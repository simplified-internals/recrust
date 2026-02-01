use quote::{ToTokens, quote};

mod utils;
use recrust_ast::BracedValue;
use utils::parse_element;

use crate::utils::{expect_element, prop_tokens};

#[test]
fn empty_tag() {
    let node = quote!(<div></div>);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert_eq!(el.props.0.len(), 0);
}

#[test]
fn empty_self_closing_tag() {
    let node = quote!(<div />);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert_eq!(el.props.0.len(), 0);
}

#[test]
fn tag_with_children_and_props() {
    let node = quote!(<div id={"main"}><span /></div>);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert!(el.props.0.keys().any(|k| k.to_string() == "id"));
    assert!(el.props.0.keys().any(|k| k.to_string() == "children"));

    // prop value is stored as `Braced` (contents of `{ ... }`)
    assert_eq!(
        prop_tokens(&el, "id").to_token_stream().to_string(),
        "\"main\""
    );

    // children are stored as a synthetic `children` prop (also a `Braced`)
    let BracedValue::Node(span_node) = &prop_tokens(&el, "children").0[0] else {
        panic!(
            "expected span node, got {:?}",
            prop_tokens(&el, "children").to_token_stream().to_string()
        );
    };

    let span_node = span_node.as_ref().clone();

    let span_node = expect_element(span_node);

    assert!(span_node.tag.to_string() == "span");
    assert_eq!(span_node.props.0.len(), 0);
}
