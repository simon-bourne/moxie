//! Wrappers around Microsoft FAST web components: https://github.com/microsoft/fast
//!
//! I'm not for one moment suggesting this lives in `moxie_dom`. It's just for playing around.
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
