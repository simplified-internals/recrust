// This won't actually compile, it's more to show how dioxus would look with the rsx! macro.
// To see the expanded code, run `cargo expand` on this file (cargo-expand is required).

use dioxus_native::prelude::*;
use recrust_macro::rsx;

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        <div>
            <div>"Counter"</div>
            <div>{count}</div>
            <button onclick={move || {count += 1}}>"Increment"</button>
            <button onclick={move || {count += 1}}>"Decrement"</button>
        </div>
    }
}

fn main() {
    dioxus_native::launch(app);
}
