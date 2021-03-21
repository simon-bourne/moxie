//! Wrappers around Microsoft FAST web components: https://github.com/microsoft/fast
//!
//! I'm not for one moment suggesting this lives in `moxie_dom`. It's just for playing around.

use augdom::event_ty;

use crate::interfaces::event_target::EventTarget;

custom_html_element! {
    fast_badge("fast-badge")

    categories { Flow }

    children {
        categories {
            Flow
        }
    }
}

custom_html_element! {
    fast_tree_view("fast-tree-view")

    categories { Flow }

    children {
        // TODO: Custom content categories
        categories {
            Flow
        }
    }
}

custom_html_element! {
    fast_tree_item("fast-tree-item")

    categories { Flow }

    children {
        // TODO: Custom content categories
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
}

event_ty! {
    /// TODO: Docs
    SelectedChangeEvent,
    "selected-change",
    sys::CustomEvent
}

event_ty! {
    /// TODO: Docs
    ExpandedChangeEvent,
    "expanded-change",
    sys::CustomEvent
}

macro_rules! event_handlers{
    ($tag:ident { $($method_name:ident <$event:ty>),* $(,)? }) => {
        paste::item!{$(
            impl [<$tag:camel Builder>] {
                /// Set an event handler
                pub fn $method_name(self, callback: impl FnMut($event) + 'static) -> Self {
                    self.on(callback)
                }
            }

            impl EventTarget<$event> for [<$tag:camel Builder>] {}
        )*}
    }
}

event_handlers!(fast_tree_item{
    on_selected_change<SelectedChangeEvent>,
    on_expanded_change<ExpandedChangeEvent>
});
