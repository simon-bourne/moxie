use moxie_dom::{
    elements::html::{div, p},
    html_element,
    interfaces::node::Parent,
    prelude::state,
};
use wasm_bindgen::prelude::wasm_bindgen;

html_element! {
    <fast-badge>

    categories { Flow }

    children {
        categories {
            Flow
        }
    }
}

html_element! {
    <fast-tree-view>

    categories { Flow }

    children {
        categories {
            Flow
        }
    }
}

html_element! {
    <fast-tree-item>

    categories { Flow }

    children {
        categories {
            Flow
        }
    }

    attributes {
        /// Is this tree item expanded?
        expanded(bool)
        /// Is this tree item selected?
        selected(bool)
    }

    custom_events {
        selected-change
        expanded-change
    }
}

impl ExpandedChange {
    /// TODO: Doc
    pub fn is_expanded(&self) -> bool {
        todo!("Read from js object")
    }
}

impl SelectedChange {
    /// TODO: Doc
    pub fn is_selected(&self) -> bool {
        todo!("Read from js object")
    }
}

#[wasm_bindgen]
pub fn boot(root: moxie_dom::raw::sys::Node) {
    moxie_dom::boot(root, || {
        let (expand_count, inc_on_expand) = state(|| 0);
        let (selected_count, inc_on_select) = state(|| 0);

        div()
            .child(fast_badge().child("Hello, world!"))
            .child(
                fast_tree_view()
                    .child(
                        fast_tree_item()
                            .on_expanded_change(move |_| inc_on_expand.mutate(|count| *count += 1))
                            .expanded(true)
                            .child("Tree item 1")
                            .child(
                                fast_tree_item()
                                    .selected(true)
                                    .child("tree item 1 - 1")
                                    .on_selected_change(move |_| {
                                        inc_on_select.mutate(|count| *count += 1)
                                    }),
                            ),
                    )
                    .child(fast_tree_item().child("Tree item 2")),
            )
            .child(format!(
                "Count of 'selected-change' events on 'Tree item 1 - 1' = {}",
                selected_count
            ))
            .child(p())
            .child(format!("Count of 'expanded-change' events on 'Tree item 1' = {}", expand_count))
    });
}
