use quote::{ToTokens, quote};

mod utils;
use utils::{parse_element, prop_tokens};

use recrust_ast::{Node, PartialExpr};

#[test]
fn plain_rust_tokens() {
    let el = parse_element(quote!(<div value={1 + 2 * 3} />));

    assert_eq!(
        prop_tokens(&el, "value").to_token_stream().to_string(),
        "1 + 2 * 3"
    );
}

#[test]
fn direct_rsx() {
    let el = parse_element(quote!(<div slot={<span />} />));

    let slot = prop_tokens(&el, "slot");
    assert_eq!(slot.0.len(), 1);

    let PartialExpr::RSX(node) = &slot.0[0] else {
        panic!("expected RSX node, got {:?}", slot);
    };

    match node.as_ref() {
        Node::Element(span) => assert_eq!(span.tag.to_string(), "span"),
        other => panic!(
            "expected `Node::Element(<span/>)`, got {:?}",
            std::mem::discriminant(other)
        ),
    }
}

#[test]
fn nested_groups() {
    let el = parse_element(quote!(<div items={{ vec![<Item />, <Item />] }} />));

    let items = prop_tokens(&el, "items").to_token_stream().to_string();

    // It should rewrite the nested rsx that is inside the `{}` and the `[]` groups.
    assert!(items.contains("vec"));
    assert!(items.contains("create_element"));
    assert!(items.contains("Item"));
}

#[test]
fn simple_braced() {
    let el = parse_element(quote!(<div>{count}</div>));

    let children = prop_tokens(&el, "children");
    assert_eq!(children.0.len(), 1);

    let PartialExpr::RSX(node) = &children.0[0] else {
        panic!(
            "expected child node, got {}",
            children.to_token_stream().to_string()
        );
    };

    match node.as_ref() {
        Node::RawExpr(expr) => assert_eq!(expr.to_token_stream().to_string(), "count"),
        other => panic!(
            "expected `Node::RawExpr`, got {:?}",
            std::mem::discriminant(other)
        ),
    }
}
