use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    LitStr, Token, braced,
    parse::{Parse, ParseStream},
};

use crate::{Braced, element::RSXElement};

#[derive(Clone)]
pub enum Node {
    Element(RSXElement),
    Braced(Braced),
    Text(LitStr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // If the next token is a <, it should be some rsx: `<div />` so parse it as an RSXElement
        if input.peek(Token![<]) {
            Ok(Node::Element(input.parse()?))
        }
        // If the next token is a {, parse its contents as a `Braced` stream
        // (mixed Rust tokens + embedded `<...>` RSX nodes).
        else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            Ok(Node::Braced(content.parse()?))
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
            Node::Braced(braced) => tokens.extend(quote::quote!({ #braced })),
            Node::Text(text) => text.to_tokens(tokens),
        }
    }
}
