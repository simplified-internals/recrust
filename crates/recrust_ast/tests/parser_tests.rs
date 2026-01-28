use quote::quote;

mod utils;
use utils::*;

#[test]
fn self_closing_tag() {
    let tokens = quote! { <div /> };

    // el
    let el = expect_element(parse_node(tokens));
    expect_tag(&el, "div");
    expect_props_len(&el, 0);
}

#[test]
fn empty_tag() {
    let tokens = quote! { <div></div> };

    let el = expect_element(parse_node(tokens));
    expect_tag(&el, "div");
    expect_props_len(&el, 0);
}

#[test]
fn attributes() {
    let tokens = quote! { <div id={"main"} class={"container"} /> };

    // el
    let el = expect_element(parse_node(tokens));
    expect_tag(&el, "div");
    expect_props_len(&el, 2);

    expect_prop_str(&el, "id", "main");
    expect_prop_str(&el, "class", "container");
}

#[test]
fn text_child() {
    let tokens = quote! { <span>"Hello"</span> };

    // el
    let el = expect_element(parse_node(tokens));
    expect_tag(&el, "span");
    expect_props_len(&el, 1);

    // children
    let children = children(&el);
    expect_children_len(&children, 1);
    expect_text(&children[0], "Hello");
}

#[test]
fn nested_elements() {
    let tokens = quote! {
        <div>
            <header />
            <footer></footer>
        </div>
    };

    // el
    let el = expect_element(parse_node(tokens));
    expect_tag(&el, "div");
    expect_props_len(&el, 1);

    // children
    let children = children(&el);
    expect_children_len(&children, 2);
    expect_child_element(&children[0], "header");
    expect_child_element(&children[1], "footer");
}

#[test]
fn mixed_children() {
    let tokens = quote! {
        <div>
            "Title: "
            <span>"Value"</span>
        </div>
    };

    // el
    let el = expect_element(parse_node(tokens));
    expect_tag(&el, "div");
    expect_props_len(&el, 1);

    // children
    let children = children(&el);
    expect_children_len(&children, 2);
    expect_text(&children[0], "Title: ");
    expect_child_element(&children[1], "span");
}
