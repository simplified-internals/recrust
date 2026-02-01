use proc_macro::TokenStream;
use quote::quote;
use recrust_ast::Node;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let node = syn::parse_macro_input!(input as Node);

    quote! {
        #node
    }
    .into()
}
