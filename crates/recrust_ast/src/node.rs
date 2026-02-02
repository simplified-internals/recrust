use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Ident, LitStr, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::{Component, Element, ExprNode};

// ---------------------------------- Macro Traits: Input / Output ----------------------------------

#[derive(Clone)]
pub enum Node {
    /// <div ... />
    Element(Element),
    /// <MyComponent ... />
    Component(Component),
    /// "Hello, world!"
    Text(LitStr),
    /// { 1 + 2 * 3 }
    /// This also finds and expands any nested RSX code inside the `{ ... }` block.
    RawExpr(ExprNode),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // `<` = element or component
        if input.peek(Token![<]) {
            let fork = input.fork();

            fork.parse::<Token![<]>()?;
            let tag = fork.call(Ident::parse_any)?.to_string();

            // If the tag is lowercase, it's an element, otherwise it's a component
            if tag.chars().next().unwrap().is_ascii_lowercase() {
                Ok(Node::Element(input.parse()?))
            } else {
                Ok(Node::Component(input.parse()?))
            }
        }
        // `{` = raw expression
        else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            Ok(Node::RawExpr(content.parse()?))
        }
        // Just a plain string literal
        else if input.peek(syn::LitStr) {
            Ok(Node::Text(input.parse()?))
        }
        // Unknown
        else {
            Err(input.error("expected element, braced block, or string literal"))
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Node::Element(element) => element.to_tokens(tokens),
            Node::Component(component) => component.to_tokens(tokens),
            // Important to add back the braces to the raw expression
            Node::RawExpr(raw_expr) => tokens.extend(quote::quote!({ #raw_expr })),
            Node::Text(text) => text.to_tokens(tokens),
        }
    }
}

// ---------------------------------- Other ----------------------------------

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Element(element) => write!(f, "Element({:?})", element),
            Node::Component(component) => write!(f, "Component({:?})", component),
            Node::RawExpr(raw_expr) => write!(f, "RawExpr({:?})", raw_expr),
            Node::Text(text) => write!(f, "Text({:?})", text.to_token_stream().to_string()),
        }
    }
}
