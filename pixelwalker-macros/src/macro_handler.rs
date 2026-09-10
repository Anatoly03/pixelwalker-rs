use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use syn::Error;
use syn::Lifetime;
use syn::{FnArg, Ident, ItemFn, Pat, Signature, Type, parse::Parse, spanned::Spanned as _};
use syn::{GenericArgument, PatType, PathArguments, TypePath, TypeReference};

/// The metadata to a handler argument.
///
/// Currently it only supports the positional parameter which is the
/// packet type, which is any packet that implements [FromWorldPacket].
///
/// # Example
///
/// ```no_run,no_test
/// #[handler(
///     Ping,
///     priority = 0,
///     error = "silent"
/// )]
/// pub fn handle_ping(ping: &Ping, channel: Channel) {
///     channel.send(ping);
/// }
/// ```
pub struct HandlerMeta {
    pub event: Ident,
}

impl Parse for HandlerMeta {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let event = input.parse()?;
        Ok(Self { event })
    }
}

#[derive(PartialEq)]
pub enum HandlerParamRole {
    /// `&mut Channel` — the websocket connection.
    Channel,
    /// `&#event` — the packet declared on `#[handler(#event)]`.
    Packet,
    /// `Res<T>` — a resource owned by the client.
    Resource(Type),
}

pub struct HandlerParam {
    ident: Ident,
    role: HandlerParamRole,
}

/// Collects function parameters into the corresponding element.
///
/// # Details
///
/// Consider the following handler signatures.
///
/// ```no_run,no_test
/// pub async fn handle_init() -> Result<()>;
/// pub async fn handle_init(channel: &mut Channel) -> Result<()>;
/// pub async fn handle_init(packet: &PlayerInitPacket, channel: &mut Channel) -> Result<()>;
/// pub async fn handle_init(channel: &mut Channel, packet: &PlayerInitPacket) -> Result<()>;
/// pub async fn handle_init(channel: &mut Channel, packet: &PlayerInitPacket, players: Res<PlayerManager>) -> Result<()>;
/// ```
///
/// The first one is the special websocket channel, it is hardcoded to be named `Channel`.
/// The second is the packet which is provided by the `#[handler()]` macro parameter, and
/// the last one is a resource managed by the client app; all in any permutation.
///
/// This function returns a vector of arguments mapped against their "role".
fn collect_function_sig(sig: &Signature, event: &Ident) -> Result<Vec<HandlerParam>, syn::Error> {
    let mut params = vec![];

    for arg in &sig.inputs {
        match arg {
            FnArg::Receiver(r) => {
                return Err(Error::new_spanned(
                    r,
                    "#[handler] functions cannot take `self`",
                ));
            }
            FnArg::Typed(pat_ty) => {
                let PatType { pat, ty, .. } = pat_ty;

                // Function signatures require to be identifiers.
                let ident = match pat.deref() {
                    Pat::Ident(pi) => pi.ident.clone(),
                    other => {
                        return Err(Error::new_spanned(
                            other,
                            "#[handler] parameters must be simple identifiers",
                        ));
                    }
                };

                // Determine the role of the function signature.
                let role = match ty.deref() {
                    // `&Channel`, `&mut Channel`, `&#event`, `&mut #event`
                    Type::Reference(TypeReference { elem, .. }) => {
                        if type_is_named(elem, "Channel") {
                            HandlerParamRole::Channel
                        } else if type_is_named(elem, &event.to_string()) {
                            HandlerParamRole::Packet
                        } else {
                            return Err(Error::new_spanned(ty, "unsupported parameter type"));
                        }
                    }
                    // `Res<T>`
                    Type::Path(TypePath { path, .. }) => {
                        let Some(last) = path.segments.last() else {
                            return Err(Error::new_spanned(ty, "unsupported parameter type"));
                        };
                        if last.ident != "Res" {
                            return Err(Error::new_spanned(
                                ty,
                                "unsupported parameter type: expected `Res<T>`, `&Channel` or `&#event`",
                            ));
                        }
                        let PathArguments::AngleBracketed(args) = &last.arguments else {
                            return Err(Error::new_spanned(
                                ty,
                                "`Res<T>` requires a type argument",
                            ));
                        };
                        let Some(GenericArgument::Type(inner)) = args.args.first() else {
                            return Err(Error::new_spanned(
                                ty,
                                "`Res<T>` requires a type argument",
                            ));
                        };
                        HandlerParamRole::Resource(inner.clone())
                    }
                    _ => return Err(Error::new_spanned(ty, "unsupported parameter type")),
                };

                params.push(HandlerParam { ident, role });
            }
        }
    }

    Ok(params)
}

/// Return `true` if `ty` is a path whose last segment is named `name`.
/// This matches both `Channel` and `some::path::Channel`.
fn type_is_named(ty: &Type, name: &str) -> bool {
    match ty {
        Type::Path(TypePath { path, .. }) => {
            path.segments.last().is_some_and(|seg| seg.ident == name)
        }
        _ => false,
    }
}

/// Inserts a `'_` lifetime into `Res<T>` if one isn't already present.
/// Returns `true` if the type was a `Res<…>` (whether or not it was modified).
fn add_res_lifetime(ty: &mut Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };
    let Some(last) = path.segments.last_mut() else {
        return false;
    };
    if last.ident != "Res" {
        return false;
    }

    let PathArguments::AngleBracketed(args) = &mut last.arguments else {
        return false;
    };

    // Already `Res<'a, T>` or `Res<'_, T>`? Leave it alone.
    if matches!(args.args.first(), Some(GenericArgument::Lifetime(_))) {
        return true;
    }

    let lt = GenericArgument::Lifetime(Lifetime::new("'_", proc_macro2::Span::call_site()));
    args.args.insert(0, lt);
    true
}

/// A macro attribute to define a packet handler.
///
/// # Example
///
/// ```no_run,no_test
/// #[handler(Ping)]
/// pub fn handle_ping(ping: &Ping, channel: Channel) {
///     channel.send(ping);
/// }
/// ```
pub(crate) fn handler(metadata: HandlerMeta, func: ItemFn) -> TokenStream {
    let HandlerMeta { event } = metadata;
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = &func;

    // Inner function is invoked with `.await`, so it must be async.
    if sig.asyncness.is_none() {
        return syn::Error::new_spanned(sig, "#[handler] function must be `async`")
            .to_compile_error();
    }

    // Collect the parameter identifiers in order.
    let params = match collect_function_sig(sig, &event) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error(),
    };
    let fn_name = &sig.ident;
    let inner_fn = Ident::new(&format!("__{}", fn_name), fn_name.span());

    // Rename the user's original function.
    let mut inner_sig = sig.clone();
    inner_sig.ident = inner_fn.clone();

    // Rewrite `Res<T>` → `Res<'_, T>` in the inner signature so users can
    // write the ergonomic form.
    for arg in &mut inner_sig.inputs {
        if let FnArg::Typed(PatType { ty, .. }) = arg {
            add_res_lifetime(ty);
        }
    }

    // Always extract the packet. If the user bound it, use their name;
    // otherwise use a hidden ident so the filter still runs.
    let packet_target = params
        .iter()
        .find_map(|p| matches!(p.role, HandlerParamRole::Packet).then(|| p.ident.clone()))
        .unwrap_or_else(|| Ident::new("__packet", proc_macro2::Span::call_site()));

    let mut extract_stmts = Vec::new();
    let mut call_args = Vec::new();

    extract_stmts.push(quote! {
        let ::core::option::Option::Some(#packet_target) =
            <#event as ::pixelwalker_api::packets::FromWorldPacket<'_>>::from_world_packet(
                world_packet,
            )
        else {
            return ::core::result::Result::Ok(());
        };
    });

    for HandlerParam { ident, role } in &params {
        match role {
            HandlerParamRole::Channel => {
                call_args.push(quote! { channel });
            }
            HandlerParamRole::Packet => {
                call_args.push(quote! { #ident });
            }
            HandlerParamRole::Resource(inner) => {
                extract_stmts.push(quote! {
                    let #ident: ::pixelwalker::client::Res<'_, #inner> =
                        match <::pixelwalker::client::Res<'_, #inner>
                            as ::pixelwalker::client::FromResources<'_>>::from_resources(
                                resources,
                            ) {
                            ::core::option::Option::Some(r) => r,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(::anyhow::anyhow!(
                                    "handler `{}` requires resource `{}`, but it was not registered",
                                    ::core::stringify!(#fn_name),
                                    ::core::stringify!(#inner),
                                ));
                            }
                        };
                });
                call_args.push(quote! { #ident });
            }
        }
    }

    let call = quote! { #inner_fn(#(#call_args),*).await };

    quote! {
        // The user's original function, renamed and hidden from the public API.
        #[doc(hidden)]
        #[allow(dead_code)]
        #vis #inner_sig #block

        // The factory. Same name as the user wrote, so `mount([handle_ping()])`
        // reads naturally.
        #(#attrs)*
        #vis fn #fn_name() -> ::std::boxed::Box<
            dyn ::pixelwalker::connection::Handler + Send + Sync,
        > {
            struct HandlerImpl;

            impl ::pixelwalker::connection::Handler for HandlerImpl {
                fn call<'a>(
                    &'a self,
                    world_packet: &'a ::pixelwalker_api::packets::WorldPacket,
                    channel: &'a mut ::pixelwalker::connection::Channel,
                    resources: &'a ::pixelwalker::connection::Resources,
                ) -> ::futures_util::future::BoxFuture<'a, ::anyhow::Result<()>> {
                    ::std::boxed::Box::pin(async move {
                        #(#extract_stmts)*
                        #call
                    })
                }
            }

            ::std::boxed::Box::new(HandlerImpl)
        }
    }
}
