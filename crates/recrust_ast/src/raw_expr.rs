use proc_macro2::{Group, Span, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::{
    Token,
    parse::{Parse, ParseStream, discouraged::Speculative},
};

use crate::node::RSXNode;

// ---------------------------------- Macro Traits: Input / Output ----------------------------------

#[derive(Clone, Debug)]
pub enum PartialExpr {
    /// Just plain rust code, no need to expand
    Normal(TokenStream2),
    /// Some RSX code which needs to be expanded
    RSX(Box<RSXNode>),
    /// Nested blocks that might contain RSX code
    ExprNode {
        delimiter: proc_macro2::Delimiter,
        span: proc_macro2::Span,
        inner: ExprNode,
    },
}

#[derive(Clone, Debug)]
pub struct ExprNode(pub Vec<PartialExpr>);

impl Parse for ExprNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        rewrite_rsx(input)
    }
}

impl ToTokens for ExprNode {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        for part in &self.0 {
            match part {
                PartialExpr::Normal(tree) => tokens.extend(tree.clone()),
                PartialExpr::RSX(node) => node.to_tokens(tokens),
                PartialExpr::ExprNode {
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

// ---------------------------------- Other ----------------------------------

/// Recursively clasifies RSX that is inside the `{ ... }` blocks
/// into **Node** and **TokenStream** tokens.
pub fn rewrite_rsx(input: ParseStream) -> syn::Result<ExprNode> {
    let mut parts: ExprNode = ExprNode(Vec::new());
    let mut current = TokenStream2::new();

    while !input.is_empty() {
        //  `<` = might be a Node
        if input.peek(Token![<]) {
            let fork = input.fork();

            if let Ok(node) = fork.parse::<RSXNode>() {
                // If the current token stream is not empty, push it to the `parts` vector
                if !current.is_empty() {
                    parts.0.push(PartialExpr::Normal(current));
                    current = TokenStream2::new();
                }

                // Parse the Node and push the expanded output to the `parts` vector
                input.advance_to(&fork);
                parts.0.push(PartialExpr::RSX(Box::new(node)));
                continue;
            }
        }

        // Retrieve the next token
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
                    parts.0.push(PartialExpr::Normal(current));
                    current = TokenStream2::new();
                }

                // parse the nested block and expand any RSX code inside it
                let inner: ExprNode = syn::parse2(g.stream())?;

                parts.0.push(PartialExpr::ExprNode {
                    delimiter: g.delimiter(),
                    span: g.span(),
                    inner,
                });
            }
            // other tokens are just added to the current token stream
            other => current.extend(std::iter::once(other)),
        }
    }

    // add any remaining tokens to the `parts` vector
    if !current.is_empty() {
        parts.0.push(PartialExpr::Normal(current));
    }

    Ok(parts)
}
