/// Dependencies used by macros
#[doc(hidden)]
pub mod __private {
    pub use paste;
    pub use crate::cached_node::CachedNode;
    pub use augdom;
    pub use crate::interfaces::node::sealed::Memoized;

    // TODO: Are these functions the best way to access private methods?
    // TODO: Do we need a topo::nested attr on this?
    pub fn cached_node_new(node: augdom::Node) -> CachedNode {
        CachedNode::new(node)
    }

    pub fn cached_node_remove_trailing_children(node: &CachedNode) {
        node.remove_trailing_children()
    }
}

/// Compute the name of the HTML attribute from the name of the builder method.
#[macro_export]
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
#[macro_export]
macro_rules! attr_method {
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident(bool)
    ) => {
        $(#[$outer])*
        $publicity fn $attr(self, to_set: bool) -> Self {
            #[allow(unused)]
            use $crate::interfaces::element::ElementBuilder;
            if to_set {
                self.attribute($crate::attr_name!($attr), "")
            } else {
                self
            }
        }
    };
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident
    ) => {
        $crate::attr_method! {
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
            use $crate::interfaces::element::ElementBuilder;
            self.attribute($crate::attr_name!($attr), to_set.to_string())
        }
    };
}

/// Define an element type.
#[macro_export]
macro_rules! element {
    (
        $(#[$outer:meta])*
        <$name:ident[$text_name:expr]>
        $(categories { $($category:ident),+ })?
        $(children {
            $(tags { $(< $child_tag:ident >),+ })?
            $(categories { $($child_category:ident),+ })?
        })?
        $(attributes {$(
            $(#[$attr_meta:meta])*
            $attr:ident $(( $attr_ty:ty ))?
        )*})?
    ) => { $crate::macros::__private::paste::item! {
        $(#[$outer])*
        
        // TODO: `topo` hygeine?
        ///
        /// A function for creating a builder which will accept attributes and produce the element.
        #[topo::nested]
        pub fn $name() -> [<$name:camel Builder>] {
            #[allow(unused)]
            use $crate::macros::__private::augdom::Dom;
            #[allow(unused)]
            use $crate::interfaces::node::NodeWrapper;

            // TODO: Does `text_name` need to match up with `name` somehow?
            let elem = moxie::cache($text_name, |ty| {
                $crate::prelude::document().create_element(ty)
            });
            [<$name:camel Builder>] { inner: $crate::macros::__private::cached_node_new(elem) }
        }

        $(#[$outer])*
        ///
        /// A type for initializing the element's attributes before calling `build`.
        #[must_use = "needs to be built"]
        pub struct [<$name:camel Builder>] {
            inner: $crate::macros::__private::CachedNode,
        }

        impl $crate::interfaces::element::ElementBuilder for [<$name:camel Builder>] {}
        impl $crate::interfaces::node::NodeWrapper for [<$name:camel Builder>] {}

        impl $crate::interfaces::node::NodeBuilder for [<$name:camel>] {
            type Target = Self;

            /// Initialize the element with all of the attributes so far.
            fn build(self) -> Self {
                self
            }
        }

        impl $crate::interfaces::node::NodeBuilder for [<$name:camel Builder>] {
            type Target = [<$name:camel>];

            /// Initialize the element with all of the attributes so far.
            fn build(self) -> [<$name:camel>] {
                use $crate::macros::__private::Memoized;
                $crate::macros::__private::cached_node_remove_trailing_children(self.node());

                [<$name:camel>] { inner: self.inner }
            }
        }

        impl $crate::macros::__private::Memoized for [<$name:camel Builder>] {
            fn node(&self) -> &$crate::macros::__private::CachedNode {
                &self.inner
            }
        }

        // children
        $(
            // child tags
            $($(
                impl $crate::interfaces::node::Parent<
                    $crate::elements::all::[<$child_tag:camel>]>
                for [< $name:camel Builder >] {}
            )+)?

            // child categories
            $($(
                impl<Child> $crate::interfaces::node::Parent<Child>
                for [< $name:camel Builder >]
                where Child: $crate::interfaces::content_categories::[<$child_category Content>] {}
            )+)?
        )?

        // attributes
        $(impl [< $name:camel Builder >] {
            $($crate::attr_method! {
                $(#[$attr_meta])*
                pub $attr $(($attr_ty))?
            })*
        })?

        $(#[$outer])*
        ///
        /// The initialized element, ready to be bound to a parent.
        #[must_use = "needs to be bound to a parent"]
        pub struct [<$name:camel>] {
            inner: $crate::macros::__private::CachedNode,
        }

        impl $crate::interfaces::node::NodeWrapper for [<$name:camel>] {}
        impl $crate::macros::__private::Memoized for [<$name:camel>] {
            fn node(&self) -> &$crate::macros::__private::CachedNode {
                &self.inner
            }
        }
        impl $crate::interfaces::element::Element for [<$name:camel>] {}

        // content categories
        $($(
            impl $crate::interfaces::content_categories::[<$category Content>]
            for [< $name:camel >] {}
        )+)?
    }};
}

// TODO: Move to lib.rs?
/// Define an HTML element type, which is essentially an `element!` with the
/// `HtmlElementBuilder` and `GlobalEventHandler` traits.
#[macro_export]
macro_rules! html_element {
    (
        $(#[$outer:meta])*
        <$name:ident>
        $($rem:tt)*
    ) => {
        html_element!{
            $(#[$outer])*
            <$name[stringify!($name)]>
            $($rem)*
        }
    };
    (
        $(#[$outer:meta])*
        <$name:ident[$text_name:expr]>
        $($rem:tt)*
    ) => { $crate::macros::__private::paste::item! {
        $crate::element! {
            $(#[$outer])*
            <$name[$text_name]>
            $($rem)*
        }

        impl $crate::interfaces::html_element::HtmlElementBuilder for [<$name:camel Builder>] {}
        impl $crate::interfaces::global_events::GlobalEventHandler for [<$name:camel Builder>] {}

        impl<E> $crate::interfaces::event_target::EventTarget<E> for [<$name:camel Builder>]
        where E: $crate::interfaces::global_events::GlobalEvent {}
    }};
}

// TODO: Make this public? + tests
macro_rules! only_text_children {
    (<$name:ident>) => {
        paste::item! {
            impl crate::interfaces::node::Parent<crate::text::Text>
            for [<$name:camel Builder>] {}
        }
    };
}
