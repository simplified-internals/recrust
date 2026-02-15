use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Ident, LitStr, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::{RSXComponent, RSXElement, ExprNode};

// ---------------------------------- Macro Traits: Input / Output ----------------------------------

#[derive(Clone)]
pub enum RSXNode {
    /// <div ... />
    RSXElement(RSXElement),
    /// <MyComponent ... />
    RSXComponent(RSXComponent),
    /// "Hello, world!"
    Text(LitStr),
    /// { 1 + 2 * 3 }
    /// This also finds and expands any nested RSX code inside the `{ ... }` block.
    RawExpr(ExprNode),
}

impl Parse for RSXNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // `<` = element or component
        if input.peek(Token![<]) {
            let fork = input.fork();

            fork.parse::<Token![<]>()?;
            let tag = fork.call(Ident::parse_any)?.to_string();

            // If the tag is lowercase, it's an element, otherwise it's a component
            if tag.chars().next().unwrap().is_ascii_lowercase() {
                Ok(RSXNode::RSXElement(input.parse()?))
            } else {
                Ok(RSXNode::RSXComponent(input.parse()?))
            }
        }
        // `{` = raw expression
        else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            Ok(RSXNode::RawExpr(content.parse()?))
        }
        // Just a plain string literal
        else if input.peek(syn::LitStr) {
            Ok(RSXNode::Text(input.parse()?))
        }
        // Unknown
        else {
            Err(input.error("expected element, braced block, or string literal"))
        }
    }
}

impl ToTokens for RSXNode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RSXNode::RSXElement(element) => element.to_tokens(tokens),
            RSXNode::RSXComponent(component) => component.to_tokens(tokens),
            // Important to add back the braces to the raw expression
            RSXNode::RawExpr(raw_expr) => tokens.extend(quote::quote!({ #raw_expr })),
            RSXNode::Text(text) => text.to_tokens(tokens),
        }
    }
}

// ---------------------------------- Other ----------------------------------

impl Debug for RSXNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSXNode::RSXElement(element) => write!(f, "Element({:?})", element),
            RSXNode::RSXComponent(component) => write!(f, "Component({:?})", component),
            RSXNode::RawExpr(raw_expr) => write!(f, "RawExpr({:?})", raw_expr),
            RSXNode::Text(text) => write!(f, "Text({:?})", text.to_token_stream().to_string()),
        }
    }
}
