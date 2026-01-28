use recrust_ast::{Node, PropValue, RSXElement};
use syn::{Expr, ExprLit, Lit};

pub fn parse_node(tokens: proc_macro2::TokenStream) -> Node {
    syn::parse2(tokens).expect("failed to parse Node")
}

pub fn expect_element(node: Node) -> RSXElement {
    match node {
        Node::Element(el) => el,
        _ => panic!("expected Element"),
    }
}

pub fn expect_tag(el: &RSXElement, expected: &str) {
    assert_eq!(el.tag.to_string(), expected);
}

pub fn expect_props_len(el: &RSXElement, expected: usize) {
    assert_eq!(el.props.len(), expected);
}

pub fn prop<'a>(el: &'a RSXElement, name: &'a str) -> &'a PropValue {
    el.props
        .iter()
        .find_map(|(k, v)| (k.to_string() == name).then_some(v))
        .unwrap_or_else(|| panic!("missing prop `{name}`"))
}

pub fn prop_opt<'a>(el: &'a RSXElement, name: &'a str) -> Option<&'a PropValue> {
    el.props
        .iter()
        .find_map(|(k, v)| (k.to_string() == name).then_some(v))
}

pub fn expect_prop_str<'a>(el: &'a RSXElement, name: &'a str, expected: &'a str) {
    let PropValue::Expr(expr) = prop(el, name) else {
        panic!("expected prop `{name}` to be Expr");
    };

    let Expr::Lit(ExprLit {
        lit: Lit::Str(s), ..
    }) = expr
    else {
        panic!("expected prop `{name}` to be a string literal expr");
    };

    assert_eq!(s.value(), expected);
}

pub fn children(el: &RSXElement) -> &Vec<Node> {
    let PropValue::Children(children) = prop(el, "children") else {
        panic!("expected `children` prop to be Children(Vec<Node>)");
    };
    children
}

pub fn expect_children_len(children: &Vec<Node>, expected: usize) {
    assert_eq!(children.len(), expected);
}

pub fn expect_text(node: &Node, expected: &str) {
    match node {
        Node::Text(s) => assert_eq!(s.value(), expected),
        _ => panic!("expected Text node"),
    }
}

pub fn expect_child_element<'a>(node: &'a Node, expected_tag: &'a str) -> &'a RSXElement {
    match node {
        Node::Element(el) => {
            assert_eq!(el.tag.to_string(), expected_tag);
            el
        }
        _ => panic!("expected Element child `{expected_tag}`"),
    }
}
