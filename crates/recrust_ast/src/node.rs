use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Ident, LitStr, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::{Component, Element, ExprNode};

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Component(Component),
    Text(LitStr),
    RawExpr(ExprNode),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // If the next token is a <, it should be some rsx: `<div />` so parse it as an RSXElement
        if input.peek(Token![<]) {
            let fork = input.fork();
            // Parse Opening Tag
            fork.parse::<Token![<]>()?;
            let tag = fork.call(Ident::parse_any)?.to_string();

            if tag.chars().next().unwrap().is_ascii_lowercase() {
                Ok(Node::Element(input.parse()?))
            } else {
                Ok(Node::Component(input.parse()?))
            }
        }
        // If the next token is a {, parse its contents as a `Braced` stream
        // (mixed Rust tokens + embedded `<...>` RSX nodes).
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
            Node::RawExpr(raw_expr) => tokens.extend(quote::quote!({ #raw_expr })),
            Node::Text(text) => text.to_tokens(tokens),
        }
    }
}

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
