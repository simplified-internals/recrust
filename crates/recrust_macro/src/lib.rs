use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use recrust_ast::Node;
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct RsxImplInput {
    runtime: Option<syn::Path>,
    node: TokenStream2,
}

impl Parse for RsxImplInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if fork.parse::<syn::Path>().is_ok() {
            let runtime = Some(input.parse::<syn::Path>()?);
            input.parse::<Token![,]>()?;
            let node = input.parse::<TokenStream2>()?;
            Ok(Self { runtime, node })
        } else {
            let node = input.parse::<TokenStream2>()?;
            Ok(Self {
                runtime: None,
                node,
            })
        }
    }
}

#[proc_macro]
pub fn rsx_impl(input: TokenStream) -> TokenStream {
    let RsxImplInput { runtime, node } = syn::parse_macro_input!(input as RsxImplInput);

    // If a runtime is provided, use it.
    if let Some(runtime) = runtime {
        quote! {
            use #runtime;
            rsx_impl!(#node)
        }
        .into()
    } else {
        // If no runtime is provided, parse the node as a Node.
        let node = TokenStream::from(node);
        let node = parse_macro_input!(node as Node);

        quote! {
            #node
        }
        .into()
    }
}
