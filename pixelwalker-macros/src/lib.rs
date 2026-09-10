use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

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
#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = func.sig.ident;
    quote! {
        pub fn #name () {}
    }
    .into()
}
