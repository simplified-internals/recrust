use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::{PartialExpr, node::Node, props::Props, raw_expr::ExprNode};

#[derive(Clone, Debug)]
pub struct Component {
    pub tag: Ident,
    pub props: Props,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse Opening Tag
        input.parse::<Token![<]>()?;
        let tag = input.call(Ident::parse_any)?;

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
        let mut children = ExprNode(Vec::new());
        while !(input.peek(Token![<]) && input.peek2(Token![/])) {
            if input.is_empty() {
                return Err(input.error("unexpected end of input while parsing children; expected a closing tag like `</...>`"));
            }

            children
                .0
                .push(PartialExpr::RSX(Box::new(input.parse::<Node>()?)));
        }
        if !children.0.is_empty() {
            props
                .0
                .insert(Ident::new("children", Span::call_site()), children);
        }

        // Parse Closing Tag: `</div>`
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let closing_tag = input.call(Ident::parse_any)?;
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

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let tag_fn = self.tag.clone();

        // key is the prop name, value is a closure that returns the prop value (enables diffing)
        let props = self.props.0.iter().map(|(key, value)| {
            let key_str = key.to_string();

            // If the value holds only nodes, we can return a vec of nodes
            if value
                .0
                .iter()
                .all(|part| matches!(part, PartialExpr::RSX(_)))
            {
                let nodes = value
                    .0
                    .iter()
                    .map(|part| match part {
                        PartialExpr::RSX(node) => quote! { #node },
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>();

                return quote! { (#key_str, vec![#(#nodes),*]) };
            }

            // Otherwise, we return the prop value as is
            quote!((#key_str, #value ))
        });
        let fn_props = quote! { vec![#(#props),*] };

        tokens.extend(quote! {
            create_element(#tag_fn, #fn_props)
        });
    }
}
