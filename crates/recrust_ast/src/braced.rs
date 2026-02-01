use proc_macro2::{Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use syn::{
    Token,
    parse::{Parse, ParseStream, discouraged::Speculative},
};

use crate::node::Node;

#[derive(Clone)]
pub enum BracedValue {
    // Just plain rust code, no need to expand
    Static(TokenStream2),
    Node(Box<Node>),
    // Nested braced blocks that might contain RSX code
    Group {
        delimiter: proc_macro2::Delimiter,
        span: proc_macro2::Span,
        inner: Braced,
    },
}

#[derive(Clone)]
pub struct Braced(pub Vec<BracedValue>);

impl Parse for Braced {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        rewrite_rsx(input)
    }
}

impl ToTokens for Braced {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for part in &self.0 {
            match part {
                BracedValue::Static(tree) => tokens.extend(tree.clone()),
                BracedValue::Node(node) => node.to_tokens(tokens),
                BracedValue::Group {
                    delimiter,
                    span,
                    inner,
                } => {
                    let mut new_g = Group::new(*delimiter, inner.to_token_stream());
                    new_g.set_span(*span);
                    tokens.extend(std::iter::once(TokenTree::Group(new_g)));
                }
            }
        }
    }
}

pub fn rewrite_rsx(input: ParseStream) -> syn::Result<Braced> {
    let mut parts: Braced = Braced(Vec::new());
    let mut current = TokenStream2::new();

    while !input.is_empty() {
        // A `<`  token might be a Node
        if input.peek(Token![<]) {
            let fork = input.fork();

            if let Ok(node) = fork.parse::<Node>() {
                // If the current token stream is not empty, push it to the parts vector
                if !current.is_empty() {
                    parts.0.push(BracedValue::Static(current));
                    current = TokenStream2::new();
                }

                // Parse the Node and push the expanded output to the parts vector
                input.advance_to(&fork);
                parts.0.push(BracedValue::Node(Box::new(node)));
                continue;
            }
        }

        // Otherwise, copy the next token tree
        // If it's a group, rewrite its inner stream so RSX inside nested blocks gets expanded too.
        let token: TokenTree = input.step(|cursor| {
            cursor
                .token_tree()
                .map(|(tt, next)| (tt, next))
                .ok_or_else(|| syn::Error::new(Span::call_site(), "unexpected end of input"))
        })?;

        match token {
            TokenTree::Group(g) => {
                // Flush any accumulated raw tokens before emitting the group to preserve order.
                if !current.is_empty() {
                    parts.0.push(BracedValue::Static(current));
                    current = TokenStream2::new();
                }

                let inner: Braced = syn::parse2(g.stream())?;

                parts.0.push(BracedValue::Group {
                    delimiter: g.delimiter(),
                    span: g.span(),
                    inner,
                });
            }
            other => current.extend(std::iter::once(other)),
        }
    }

    if !current.is_empty() {
        parts.0.push(BracedValue::Static(current));
    }

    Ok(parts)
}
