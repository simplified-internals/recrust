use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::{
    Token, braced,
    parse::{Parse, ParseStream, Parser},
};

use crate::element::RSXElement;

pub enum Node {
    Element(RSXElement),
    Braced(TokenStream),
    Text(String),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            Ok(Node::Element(input.parse()?))
        } else if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            Ok(Node::Braced(content.parse()?))
        } else if input.peek(syn::LitStr) {
            let lit: syn::LitStr = input.parse()?;
            Ok(Node::Text(lit.value()))
        } else {
            Err(input.error("expected element or string literal"))
        }
    }
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Node::Text(text) => {
                tokens.extend(quote! { #text });
            }
            Node::Element(el) => {
                let tag = &el.tag;

                let prop_tuples = el.props.iter().map(|(k, v)| {
                    let key = proc_macro2::Literal::string(&k.to_string());
                    quote! {
                        (#key,#v)
                    }
                });

                tokens.extend(quote! {
                    {
                        let mut __rsx_props = ::std::collections::HashMap::from([#(#prop_tuples),*]);
                        create_element(#tag, __rsx_props)
                    }
                });
            }
            Node::Braced(braced) => {
                let rewritten = { rewrite_rsx(braced.clone()) };

                tokens.extend(quote!({ #rewritten }));
            }
        }
    }
}

// Loops through all tokens in the TokenStream
// When encountering a <, try to parse as a Node
// If successful, replace the < with the expanded output for that node
// If not, continue parsing the stream
pub fn rewrite_rsx(stream: TokenStream) -> TokenStream {
    // Handles nested blocks
    fn rewrite_group(g: Group) -> Group {
        let mut new_g = Group::new(g.delimiter(), rewrite_rsx(g.stream()));
        new_g.set_span(g.span());
        new_g
    }

    let parser = |input: ParseStream<'_>| -> syn::Result<TokenStream> {
        let mut parts: Vec<TokenStream> = Vec::new();
        let mut current = TokenStream::new();

        while !input.is_empty() {
            // If the next token is a <, try to parse as a Node
            if input.peek(Token![<]) {
                let fork = input.fork();
                if fork.parse::<Node>().is_ok() {
                    // If the current token stream is not empty, push it to the parts vector
                    if !current.is_empty() {
                        parts.push(current);
                        current = TokenStream::new();
                    }

                    // Parse the Node and push the expanded output to the parts vector
                    let node = input.parse::<Node>()?;
                    parts.push(quote!(#node));
                    continue;
                }
            }

            let tt: TokenTree = input.step(|cursor| {
                cursor
                    .token_tree()
                    .map(|(tt, next)| (tt, next))
                    .ok_or_else(|| syn::Error::new(Span::call_site(), "unexpected end of input"))
            })?;

            match tt {
                TokenTree::Group(g) => {
                    current.extend(std::iter::once(TokenTree::Group(rewrite_group(g))))
                }
                other => current.extend(std::iter::once(other)),
            }
        }

        if !current.is_empty() {
            parts.push(current);
        }

        let mut out = TokenStream::new();
        for part in parts {
            out.extend(part);
        }
        Ok(out)
    };

    match parser.parse2(stream) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}
