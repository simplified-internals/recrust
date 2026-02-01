use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    parse::{Parse, ParseStream},
};

use crate::{
    braced::{Braced, BracedValue},
    node::Node,
    props::Props,
};

#[derive(Clone)]
pub struct RSXElement {
    pub tag: Ident,
    pub props: Props,
}

impl Parse for RSXElement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse Opening Tag
        input.parse::<Token![<]>()?;
        let tag = input.parse::<Ident>()?;

        // Parse props
        let mut props = input.parse::<Props>()?;

        // Handle self-closing tags: `<div />`
        if input.peek(Token![/]) && input.peek2(Token![>]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            return Ok(Self { tag, props });
        }

        // Handle normal opening tags: `<div>`
        input.parse::<Token![>]>()?;

        // Parse children
        let mut children = Braced(Vec::new());
        while !(input.peek(Token![<]) && input.peek2(Token![/])) {
            if input.is_empty() {
                return Err(input.error("unexpected end of input while parsing children; expected a closing tag like `</...>`"));
            }

            children
                .0
                .push(BracedValue::Node(Box::new(input.parse::<Node>()?)));
        }
        if !children.0.is_empty() {
            props
                .0
                .insert(Ident::new("children", Span::call_site()), children);
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

impl ToTokens for RSXElement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_fn = self.tag.clone();

        // Conver props to a HashMap of key-value pairs
        // key is the prop name, value is a closure that returns the prop value (enables diffing)
        let props = self.props.0.iter().map(|(k, v)| quote!((#k, || #v)));
        let fn_props = quote! { std::collections::HashMap::from([#(#props),*]) };

        tokens.extend(quote! {
            create_element(#tag_fn, #fn_props)
        });
    }
}
