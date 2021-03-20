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
