use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use proc_macro2::TokenStream;
use syn::{
    Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use crate::rewrite_rsx;

pub struct Props(HashMap<Ident, TokenStream>);

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

impl Deref for Props {
    type Target = HashMap<Ident, TokenStream>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Props {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Prop {
    pub name: Ident,
    pub value: TokenStream,
}

impl Parse for Prop {
    // ident = { expr }
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![=]>()?;

        let content;
        braced!(content in input);

        Ok(Self {
            name,
            value: rewrite_rsx(content.parse()?),
        })
    }
}
