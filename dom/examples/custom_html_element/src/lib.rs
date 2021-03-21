// TODO: `topo` hygeine
use moxie_dom::{html_element, interfaces::node::Parent};
use wasm_bindgen::prelude::wasm_bindgen;

html_element!(
    <my_custom_element>

    categories {
        Flow, Palpable
    }

    children {
        categories {
            Flow
        }
    }
);

#[wasm_bindgen]
pub fn boot(root: moxie_dom::raw::sys::Node) {
    moxie_dom::boot(root, || my_custom_element().child("Hello, world!!!"));
}
