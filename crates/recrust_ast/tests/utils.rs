use proc_macro2::TokenStream;
use recrust_ast::{RSXElement, RSXNode};

pub fn parse_node(tokens: TokenStream) -> RSXNode {
    syn::parse2(tokens).expect("failed to parse Node")
}

pub fn expect_element(node: RSXNode) -> RSXElement {
    match node {
        RSXNode::RSXElement(el) => el,
        other => panic!(
            "expected Element node, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

pub fn parse_element(tokens: TokenStream) -> RSXElement {
    expect_element(parse_node(tokens))
}

pub fn prop_tokens<'a>(el: &'a RSXElement, name: &'a str) -> &'a recrust_ast::ExprNode {
    use recrust_ast::RSXAttribute;
    el.attributes
        .0
        .iter()
        .find_map(|attr| match attr {
            RSXAttribute::Normal { name: n, value } if n.to_string() == name => Some(value),
            _ => None,
        })
        .unwrap_or_else(|| panic!("missing prop `{name}`"))
}
