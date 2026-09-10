use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, Pat, parse::Parse, spanned::Spanned as _};

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
    let mut param_idents = Vec::new();
    for arg in &sig.inputs {
        match arg {
            FnArg::Receiver(r) => {
                return syn::Error::new_spanned(r, "#[handler] functions cannot take `self`")
                    .to_compile_error();
            }
            FnArg::Typed(pat_ty) => match &*pat_ty.pat {
                Pat::Ident(pi) => param_idents.push(pi.ident.clone()),
                other => {
                    return syn::Error::new_spanned(
                        other,
                        "#[handler] parameters must be simple identifiers",
                    )
                    .to_compile_error();
                }
            },
        }
    }

    // Supported shapes: `(channel)` or `(packet, channel)`.
    let (packet_ident, channel_ident) = match param_idents.len() {
        1 => (None, param_idents[0].clone()),
        2 => (Some(param_idents[0].clone()), param_idents[1].clone()),
        _ => {
            return syn::Error::new(
                sig.inputs.span(),
                "#[handler] expects `(channel)` or `(packet, channel)`",
            )
            .to_compile_error();
        }
    };

    let fn_name = &sig.ident;
    let inner_fn = Ident::new(&format!("__{}", fn_name), fn_name.span());

    // Rename the user's original function.
    let mut inner_sig = sig.clone();
    inner_sig.ident = inner_fn.clone();

    // Call the user's function with the right arguments.
    let call = match &packet_ident {
        Some(p) => quote! { #inner_fn(#p, #channel_ident).await },
        None => quote! { #inner_fn(#channel_ident).await },
    };
    // Extract the packet from the world packet, bailing out silently if the
    // incoming variant doesn't match this handler. We go through the trait
    // method rather than matching on the protobuf enum, so the macro stays
    // agnostic to the packet's shape.
    let extract = match &packet_ident {
        Some(p) => quote! {
            let ::core::option::Option::Some(#p) =
                <#event as ::pixelwalker_api::packets::FromWorldPacket>::from_world_packet(world_packet)
            else {
                return ::core::result::Result::Ok(());
            };
        },
        None => quote! {
            if <#event as ::pixelwalker_api::packets::FromWorldPacket>::from_world_packet(world_packet)
                .is_none()
            {
                return ::core::result::Result::Ok(());
            }
        },
    };

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
                ) -> ::futures_util::future::BoxFuture<'a, ::anyhow::Result<()>> {
                    ::std::boxed::Box::pin(async move {
                        #extract
                        #call
                    })
                }
            }

            ::std::boxed::Box::new(HandlerImpl)
        }
    }
}
