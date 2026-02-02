use proc_macro2::TokenStream;
use recrust_ast::{Element, Node};

pub fn parse_node(tokens: TokenStream) -> Node {
    syn::parse2(tokens).expect("failed to parse Node")
}

pub fn expect_element(node: Node) -> Element {
    match node {
        Node::Element(el) => el,
        other => panic!(
            "expected Element node, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

pub fn parse_element(tokens: TokenStream) -> Element {
    expect_element(parse_node(tokens))
}

pub fn prop_tokens<'a>(el: &'a Element, name: &'a str) -> &'a recrust_ast::ExprNode {
    el.props
        .0
        .iter()
        .find_map(|(k, v)| (k.to_string() == name).then_some(v))
        .unwrap_or_else(|| panic!("missing prop `{name}`"))
}
