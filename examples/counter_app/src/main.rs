// This won't actually compile, it's more to show how dioxus would look with the rsx! macro.
// To see the expanded code, run `cargo expand` on this file (cargo-expand is required).

use counter_app::create_element;
use dioxus_native::prelude::*;
use recrust_macro::rsx;

#[component]
fn App() -> Element {
    let count = use_signal(|| 0);

    rsx! {
        <div>
            <div>{count.read()}</div>
            <button onclick={move || {let mut c = count.write(); *c += 1;}}>"Increment"</button>
            <button onclick={move || {let mut c = count.write(); *c -= 1;}}>"Decrement"</button>
        </div>
    }
}

fn main() {
    dioxus_native::launch(App);
}
