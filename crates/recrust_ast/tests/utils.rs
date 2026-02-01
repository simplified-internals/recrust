use proc_macro2::TokenStream;
use recrust_ast::{Node, RSXElement};

pub fn parse_node(tokens: TokenStream) -> Node {
    syn::parse2(tokens).expect("failed to parse Node")
}

pub fn expect_element(node: Node) -> RSXElement {
    match node {
        Node::Element(el) => el,
        other => panic!(
            "expected Element node, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

pub fn parse_element(tokens: TokenStream) -> RSXElement {
    expect_element(parse_node(tokens))
}

pub fn prop_tokens<'a>(el: &'a RSXElement, name: &'a str) -> &'a recrust_ast::Braced {
    el.props
        .0
        .iter()
        .find_map(|(k, v)| (k.to_string() == name).then_some(v))
        .unwrap_or_else(|| panic!("missing prop `{name}`"))
}
