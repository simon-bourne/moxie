// TODO: `topo` hygeine
use moxie_dom::{elements::html::div, html_element, interfaces::node::Parent};
use wasm_bindgen::prelude::wasm_bindgen;

html_element!(
    <my_custom_element["my-custom-element"]>

    categories {
        Flow, Palpable
    }

    children {
        categories {
            Flow
        }
    }

    attributes {
        bool_attr(bool)
        u32_attr(u32)
        stringly_typed_attr
    }
);

html_element!(
    <my_custom_div_wrapper["my-custom-div-wrapper"]>

    categories {
        Flow, Palpable
    }

    children {
        tags {
            <div>
        }
    }
);

#[wasm_bindgen]
pub fn boot(root: moxie_dom::raw::sys::Node) {
    moxie_dom::boot(root, || {
        div()
            .child(
                my_custom_element()
                    .bool_attr(true)
                    .u32_attr(1234)
                    .stringly_typed_attr("abc")
                    .child("Hello, world!!!"),
            )
            .child(my_custom_div_wrapper().child(div()))
    });
}
