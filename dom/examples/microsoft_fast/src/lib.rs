use moxie_dom::{elements::html::*, microsoft_fast::fast_badge, prelude::*};
use wasm_bindgen::prelude::*;

/// The counter_fn example, but using the DOM builder API.
#[wasm_bindgen]
pub fn boot(root: moxie_dom::raw::sys::Node) {
    moxie_dom::boot(root, || div().child(fast_badge().child("Hello, world!")));
}
