use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use syn::{
    Expr, Ident, Token, braced,
    parse::{Parse, ParseStream},
};

use crate::node::Node;

pub struct Props(HashMap<Ident, PropValue>);

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
    type Target = HashMap<Ident, PropValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Props {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum PropValue {
    Expr(Expr),
    Children(Vec<Node>),
}

pub struct Prop {
    pub name: Ident,
    pub value: PropValue,
}

impl Parse for Prop {
    // ident = { expr }
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;

        input.parse::<Token![=]>()?;

        let content;
        braced!(content in input);

        let value: PropValue = if name.to_string() == "children" {
            let mut children = Vec::new();
            while !content.is_empty() {
                children.push(content.parse::<Node>()?);
            }
            PropValue::Children(children)
        } else {
            PropValue::Expr(content.parse::<Expr>()?)
        };

        Ok(Self { name, value })
    }
}
