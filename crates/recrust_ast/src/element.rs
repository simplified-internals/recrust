use proc_macro2::Span;
use quote::quote;
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

use crate::{node::Node, props::Props};

pub struct RSXElement {
    pub tag: Ident,
    pub props: Props,
}

impl Parse for RSXElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse Opening Tag
        input.parse::<Token![<]>()?;
        let tag = input.parse::<Ident>()?;

        // Parse Props
        let mut props = input.parse::<Props>()?;

        // Self-closing tags: `<div />`
        if input.peek(Token![/]) && input.peek2(Token![>]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            return Ok(Self { tag, props });
        }

        // Normal opening tag: `<div>`
        input.parse::<Token![>]>()?;

        // Parse Children
        let mut children = Vec::new();
        while !(input.peek(Token![<]) && input.peek2(Token![/])) {
            if input.is_empty() {
                return Err(input.error("unexpected end of input while parsing children; expected a closing tag like `</...>`"));
            }

            children.push(input.parse::<Node>()?);
        }

        // Only insert `children` if the element actually has children.
        if !children.is_empty() {
            props.insert(
                Ident::new("children", Span::call_site()),
                quote! { vec![#(#children),*] },
            );
        }

        // Parse Closing Tag: `</div>`
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let closing_tag = input.parse::<Ident>()?;
        input.parse::<Token![>]>()?;

        if closing_tag != tag {
            return Err(syn::Error::new(
                closing_tag.span(),
                format!("mismatched closing tag: expected `</{}>`", tag),
            ));
        }

        Ok(Self { tag, props })
    }
}
