use std::collections::HashMap;

use syn::{
    Ident, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::raw_expr::ExprNode;

// ---------------------------------- Macro Traits: Input / Output ----------------------------------

#[derive(Clone, Debug)]
pub struct Props(pub HashMap<Ident, ExprNode>);

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

#[derive(Clone, Debug)]
pub struct Prop {
    pub name: Ident,
    pub value: ExprNode,
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.call(Ident::parse_any)?;

        input.parse::<Token![=]>()?;

        let content;
        braced!(content in input);

        Ok(Self {
            name,
            value: content.parse()?,
        })
    }
}
