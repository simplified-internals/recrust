use syn::{
    Ident, Token, braced,
    ext::IdentExt,
    parse::{Parse, ParseStream},
};

use crate::raw_expr::ExprNode;

// ---------------------------------- Macro Traits: Input / Output ----------------------------------

#[derive(Clone, Debug)]
pub struct RSXAttributes(pub Vec<RSXAttribute>);

impl Parse for RSXAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = Vec::new();

        while !(input.peek(Token![>]) || (input.peek(Token![/]) && input.peek2(Token![>]))) {
            let attribute = input.parse::<RSXAttribute>()?;
            attributes.push(attribute);
        }

        Ok(Self(attributes))
    }
}

#[derive(Clone, Debug)]
pub enum RSXAttribute {
    Normal { name: Ident, value: ExprNode },
    Spread { ident: Ident },
}

impl Parse for RSXAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Spread: {..Attributes}
        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            content.parse::<Token![..]>()?;
            return Ok(Self::Spread {
                ident: content.parse()?,
            });
        }

        // Normal: attrib_name = { ... }
        let name = input.call(Ident::parse_any)?;

        input.parse::<Token![=]>()?;

        let content;
        braced!(content in input);

        Ok(Self::Normal {
            name,
            value: content.parse()?,
        })
    }
}
