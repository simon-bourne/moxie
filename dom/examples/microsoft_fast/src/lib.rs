use moxie_dom::{
    elements::html::*,
    microsoft_fast::{fast_badge, fast_tree_item, fast_tree_view},
    prelude::*,
};
use wasm_bindgen::prelude::*;

/// The counter_fn example, but using the DOM builder API.
#[wasm_bindgen]
pub fn boot(root: moxie_dom::raw::sys::Node) {
    moxie_dom::boot(root, || {
        div().child(fast_badge().child("Hello, world!")).child(
            fast_tree_view()
                .child(
                    fast_tree_item()
                        .expanded(true)
                        .child("Tree item 1")
                        .child(fast_tree_item().selected(true).child("tree item 1 - 1")),
                )
                .child(fast_tree_item().child("Tree item 2")),
        )
    });
}
