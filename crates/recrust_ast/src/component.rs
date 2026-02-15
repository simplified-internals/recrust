use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident, Token,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::{
    PartialExpr, RSXAttribute, attributes::RSXAttributes, node::RSXNode, raw_expr::ExprNode,
};

// ---------------------------------- Macro Traits: Input / Output ----------------------------------

#[derive(Clone, Debug)]
pub enum RSXComponent {
    Normal {
        opening_tag: Ident,
        attributes: RSXAttributes,
        closing_tag: Ident,
    },
    SelfClosing {
        tag: Ident,
        attributes: RSXAttributes,
    },
}

impl Parse for RSXComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse Opening Tag
        input.parse::<Token![<]>()?;
        let tag = input.call(Ident::parse_any)?;

        // Parse attributes
        let mut attributes = input.parse::<RSXAttributes>()?;

        // Handle self-closing tags: `<div />`
        if input.peek(Token![/]) && input.peek2(Token![>]) {
            input.parse::<Token![/]>()?;
            input.parse::<Token![>]>()?;
            return Ok(RSXComponent::SelfClosing { tag, attributes });
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
                .push(PartialExpr::RSX(Box::new(input.parse::<RSXNode>()?)));
        }
        if !children.0.is_empty() {
            attributes.0.push(RSXAttribute::Normal {
                name: Ident::new("children", Span::call_site()),
                value: children,
            });
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

        Ok(RSXComponent::Normal {
            opening_tag: tag,
            attributes,
            closing_tag,
        })
    }
}

impl ToTokens for RSXComponent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let (tag, attributes) = match self {
            RSXComponent::Normal {
                opening_tag,
                attributes,
                closing_tag: _,
            } => (opening_tag, attributes),
            RSXComponent::SelfClosing { tag, attributes } => (tag, attributes),
        };

        let tag_fn = tag.clone();

        let attributes = attributes.0.iter().map(|attribute| match attribute {
            RSXAttribute::Normal { name, value } => {
                let name_str = name.to_string();
                quote! { __attrs.push( (#name_str, #value) ); }
            }
            RSXAttribute::Spread { ident } => {
                quote! { __attrs.extend( #ident ); }
            }
        });

        tokens.extend(quote! {
            create_element(#tag_fn, {
                let mut __attrs = Vec::new();

                #(#attributes)*

                __attrs
            }
            )
        });
    }
}
