use std::collections::HashMap;

use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use crate::braced::Braced;

#[derive(Clone)]
pub struct Props(pub HashMap<Ident, Braced>);

impl Parse for Props {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut props = HashMap::new();
        while !(input.peek(Token![>]) || (input.peek(Token![/]) && input.peek2(Token![>]))) {
            let prop = input.parse::<Prop>()?;
            props.insert(prop.name, prop.value);
        }
        Ok(Self(props))
    }
}

#[derive(Clone)]
pub struct Prop {
    pub name: Ident,
    pub value: Braced,
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![=]>()?;

        let content;
        braced!(content in input);

        Ok(Self {
            name,
            value: content.parse()?,
        })
    }
}
