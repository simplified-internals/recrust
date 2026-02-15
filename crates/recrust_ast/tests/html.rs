use quote::{ToTokens, quote};

mod utils;
use recrust_ast::{PartialExpr, RSXAttribute};
use utils::parse_element;

use crate::utils::{expect_element, prop_tokens};

#[test]
fn empty_tag() {
    let node = quote!(<div></div>);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert_eq!(el.attributes.0.len(), 0);
}

#[test]
fn empty_self_closing_tag() {
    let node = quote!(<div />);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert_eq!(el.attributes.0.len(), 0);
}

#[test]
fn tag_with_children_and_attributes() {
    let node = quote!(<div id={"main"}><span /></div>);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert!(el.attributes.0.iter().any(
        |attr| matches!(attr, RSXAttribute::Normal { name, value } if name.to_string() == "id")
    ));
    assert!(
        el.attributes
            .0
            .iter()
            .any(|attr| matches!(attr, RSXAttribute::Normal { name, value } if name.to_string() == "children"))
    );

    // prop value is stored as `Braced` (contents of `{ ... }`)
    assert_eq!(
        prop_tokens(&el, "id").to_token_stream().to_string(),
        "\"main\""
    );

    // children are stored as a synthetic `children` prop (also a `Braced`)
    let PartialExpr::RSX(span_node) = &prop_tokens(&el, "children").0[0] else {
        panic!(
            "expected span node, got {:?}",
            prop_tokens(&el, "children").to_token_stream().to_string()
        );
    };

    let span_node = span_node.as_ref().clone();

    let span_node = expect_element(span_node);

    assert!(span_node.tag.to_string() == "span");
    assert_eq!(span_node.attributes.0.len(), 0);
}

#[test]
fn spread_attributes() {
    let node = quote!(<div {..attrs} />);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert_eq!(el.attributes.0.len(), 1);
    assert!(matches!(
        &el.attributes.0[0],
        RSXAttribute::Spread { ident } if ident.to_string() == "attrs"
    ));

    // ToTokens should emit __attrs.extend(attrs)
    let tokens = el.to_token_stream().to_string();
    assert!(tokens.contains("__attrs"));
    assert!(tokens.contains("extend"));
    assert!(tokens.contains("attrs"));
}

#[test]
fn mix_normal_and_spread_attributes() {
    let node = quote!(<div id={"main"} {..extra} />);
    let el = parse_element(node);

    assert_eq!(el.tag.to_string(), "div");
    assert_eq!(el.attributes.0.len(), 2);

    // First: Normal id attribute
    assert!(matches!(
        &el.attributes.0[0],
        RSXAttribute::Normal { name, .. } if name.to_string() == "id"
    ));

    // Second: Spread
    assert!(matches!(
        &el.attributes.0[1],
        RSXAttribute::Spread { ident } if ident.to_string() == "extra"
    ));
}
