use syn::{
    LitStr, Token,
    parse::{Parse, ParseStream},
};

use crate::element::RSXElement;

pub enum Node {
    Element(RSXElement),
    Text(LitStr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            Ok(Node::Element(input.parse()?))
        } else if input.peek(LitStr) {
            Ok(Node::Text(input.parse()?))
        } else {
            Err(input.error("expected element or string literal"))
        }
    }
}
