use proc_macro::TokenStream;

#[proc_macro_derive(Dash, attributes(dash))]
pub fn derive_dash(ts: TokenStream) -> TokenStream {
    ts
}