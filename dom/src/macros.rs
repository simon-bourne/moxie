/// Compute the name of the HTML attribute from the name of the builder method.
macro_rules! attr_name {
    (accept_charset) => {
        "accept-charset"
    };
    (as_) => {
        "as"
    };
    (async_) => {
        "async"
    };
    (for_) => {
        "for"
    };
    (http_equiv) => {
        "http-equiv"
    };
    (current_time) => {
        "currentTime"
    };
    (loop_) => {
        "loop"
    };
    (type_) => {
        "type"
    };
    ($attr:ident) => {
        stringify!($attr)
    };
}

/// Stamps a *string* attribute method with the provided identifier as the name,
/// optionally passing docs.
macro_rules! attr_method {
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident(bool)
    ) => {
        $(#[$outer])*
        $publicity fn $attr(self, to_set: bool) -> Self {
            #[allow(unused)]
            use crate::interfaces::element::ElementBuilder;
            if to_set {
                self.attribute(attr_name!($attr), "")
            } else {
                self
            }
        }
    };
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident
    ) => {
        attr_method! {
            $(#[$outer])*
            $publicity $attr(impl ToString)
        }
    };
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident($arg:ty)
    ) => {
        $(#[$outer])*
        $publicity fn $attr(self, to_set: $arg) -> Self {
            #[allow(unused)]
            use crate::interfaces::element::ElementBuilder;
            self.attribute(attr_name!($attr), to_set.to_string())
        }
    };
}

/// Define an element type.
macro_rules! element {
    (
        $(#[$outer:meta])*
        $fn_name:ident
        ($markup_name:expr)
        $(categories { $($category:ident),+ })?
        $(children {
            $(tags { $(< $child_tag:ident >),+ })?
            $(categories { $($child_category:ident),+ })?
        })?
        $(attributes {$(
            $(#[$attr_meta:meta])*
            $attr:ident $(( $attr_ty:ty ))?
        )*})?
    ) => { paste::item! {
        $(#[$outer])*
        ///
        /// A function for creating a builder which will accept attributes and produce the element.
        #[topo::nested]
        pub fn $fn_name() -> [<$fn_name:camel Builder>] {
            #[allow(unused)]
            use augdom::Dom;
            #[allow(unused)]
            use crate::interfaces::node::NodeWrapper;

            let elem = moxie::cache($markup_name, |ty| {
                $crate::prelude::document().create_element(ty)
            });
            [<$fn_name:camel Builder>] { inner: crate::cached_node::CachedNode::new(elem) }
        }

        $(#[$outer])*
        ///
        /// A type for initializing the element's attributes before calling `build`.
        #[must_use = "needs to be built"]
        pub struct [<$fn_name:camel Builder>] {
            inner: crate::cached_node::CachedNode,
        }

        impl crate::interfaces::element::ElementBuilder for [<$fn_name:camel Builder>] {}
        impl crate::interfaces::node::NodeWrapper for [<$fn_name:camel Builder>] {}

        impl $crate::interfaces::node::NodeBuilder for [<$fn_name:camel>] {
            type Target = Self;

            /// Initialize the element with all of the attributes so far.
            fn build(self) -> Self {
                self
            }
        }

        impl $crate::interfaces::node::NodeBuilder for [<$fn_name:camel Builder>] {
            type Target = [<$fn_name:camel>];

            /// Initialize the element with all of the attributes so far.
            fn build(self) -> [<$fn_name:camel>] {
                use crate::interfaces::node::sealed::Memoized;
                self.node().remove_trailing_children();

                [<$fn_name:camel>] { inner: self.inner }
            }
        }

        impl crate::interfaces::node::sealed::Memoized for [<$fn_name:camel Builder>] {
            fn node(&self) -> &crate::cached_node::CachedNode {
                &self.inner
            }
        }

        // children
        $(
            // child tags
            $($(
                impl crate::interfaces::node::Parent<
                    crate::elements::just_all_of_it_ok::[<$child_tag:camel>]>
                for [< $fn_name:camel Builder >] {}
            )+)?

            // child categories
            $($(
                impl<Child> crate::interfaces::node::Parent<Child>
                for [< $fn_name:camel Builder >]
                where Child: crate::interfaces::content_categories::[<$child_category Content>] {}
            )+)?
        )?

        // attributes
        $(impl [< $fn_name:camel Builder >] {
            $(attr_method! {
                $(#[$attr_meta])*
                pub $attr $(($attr_ty))?
            })*
        })?

        $(#[$outer])*
        ///
        /// The initialized element, ready to be bound to a parent.
        #[must_use = "needs to be bound to a parent"]
        pub struct [<$fn_name:camel>] {
            inner: crate::cached_node::CachedNode,
        }

        impl crate::interfaces::node::NodeWrapper for [<$fn_name:camel>] {}
        impl crate::interfaces::node::sealed::Memoized for [<$fn_name:camel>] {
            fn node(&self) -> &crate::cached_node::CachedNode {
                &self.inner
            }
        }
        impl crate::interfaces::element::Element for [<$fn_name:camel>] {}

        // content categories
        $($(
            impl crate::interfaces::content_categories::[<$category Content>]
            for [< $fn_name:camel >] {}
        )+)?
    }};
}

// TODO: Export this?
// TODO: How does this interact with `mox!`?
macro_rules! custom_html_element {
    (
        $(#[$outer:meta])*
        $fn_name:ident
        ($markup_name:expr)
        $($rem:tt)*
    ) => { paste::item! {
        element! {
            $(#[$outer])*
            $fn_name
            ($markup_name)
            $($rem)*
        }

        impl crate::interfaces::html_element::HtmlElementBuilder for [<$fn_name:camel Builder>] {}
        impl crate::interfaces::global_events::GlobalEventHandler for [<$fn_name:camel Builder>] {}

        impl<E> crate::interfaces::event_target::EventTarget<E> for [<$fn_name:camel Builder>]
        where E: crate::interfaces::global_events::GlobalEvent {}
    }};
}

/// Define an HTML element type, which is essentially an `element!` with the
/// `HtmlElementBuilder` and `GlobalEventHandler` traits.
macro_rules! html_element {
    (
        $(#[$outer:meta])*
        <$name:ident>
        $($rem:tt)*
    ) => {
        custom_html_element!($(#[$outer])* $name (stringify!($name)) $($rem)*);
    }
}

macro_rules! only_text_children {
    (<$name:ident>) => {
        paste::item! {
            impl crate::interfaces::node::Parent<crate::text::Text>
            for [<$name:camel Builder>] {}
        }
    };
}
