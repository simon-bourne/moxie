// TODO: `topo` hygeine
use moxie_dom::html_element;
use wasm_bindgen::prelude::*;

html_element!(
    <my_custom_element>
);

#[wasm_bindgen]
pub fn boot(root: moxie_dom::raw::sys::Node) {
    moxie_dom::boot(root, my_custom_element);
}
