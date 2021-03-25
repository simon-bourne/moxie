// Dependencies used by macros
#[doc(hidden)]
pub mod __private {
    pub use crate::cached_node::CachedNode;
    pub use crate::interfaces::node::sealed::Memoized;
    pub use paste;
    pub mod just_all_of_it_ok {
        pub use crate::elements::{
            embedding::*, forms::*, interactive::*, media::*, metadata::*, scripting::*,
            sectioning::*, table::*, text_content::*, text_semantics::*, *,
        };
    }
    pub use augdom::custom_event;

    pub fn cache_elem(name: &str) -> CachedNode {
        #[allow(unused)]
        use crate::interfaces::node::NodeWrapper;
        #[allow(unused)]
        use augdom::Dom;

        let elem = moxie::cache(name, |ty| crate::prelude::document().create_element(ty));

        CachedNode::new(elem)
    }

    pub fn remove_trailing_children(node: &impl Memoized) {
        node.node().remove_trailing_children()
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! name_as_str {
    ($name:ident $(- $name_tail:ident)*) => {
        concat!(stringify!($name) $(, "-", stringify!($name_tail))*)
    }
}

#[doc(hidden)]
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
    ($attr:ident $(- $attr_tail:ident)*) => {
        $crate::name_as_str!($attr $(- $attr_tail)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! named_attr_method {
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident [$name:expr] (bool)
    ) => {
        $(#[$outer])*
        $publicity fn $attr(self, to_set: bool) -> Self {
            #[allow(unused)]
            use $crate::interfaces::element::ElementBuilder;
            if to_set {
                self.attribute($name, "")
            } else {
                self
            }
        }
    };
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident [$name:expr] ($arg:ty)
    ) => {
        $(#[$outer])*
        $publicity fn $attr(self, to_set: $arg) -> Self {
            #[allow(unused)]
            use $crate::interfaces::element::ElementBuilder;
            self.attribute($name, to_set.to_string())
        }
    };
}

#[doc(hidden)]
/// Stamps a *string* attribute method with the provided identifier as the name,
/// optionally passing docs.
#[macro_export]
macro_rules! attr_method {
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident $(- $attr_tail:ident)*
    ) => { $crate::macros::__private::paste::item! {
        $crate::attr_method! {
            $(#[$outer])*
            $publicity
                $attr $(- $attr_tail)*
                (impl ToString)
        }
    }};
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident $(- $attr_tail:ident)* ($($rest:tt)*)
    ) => { $crate::macros::__private::paste::item! {
        $crate::named_attr_method! {
            $(#[$outer])*
            $publicity
                [<$attr $(_ $attr_tail)*>]
                [$crate::attr_name!($attr $(- $attr_tail)*)]
                ($($rest)*)
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! custom_event {
    (
        $tag_name:ident {
            $(#[$event_meta:meta])*
            $event_type:ident($event_ty_str:expr)
        }
    ) => { $crate::macros::__private::paste::item! {
        $crate::macros::__private::custom_event!(
            $(#[$event_meta])*[<$event_type:camel>], $event_ty_str
        );

        impl [<$tag_name:camel Builder>] {
            /// Set an event handler
            pub fn [<on_ $event_type>](self, callback: impl FnMut([<$event_type:camel>]) + 'static) -> Self {
                use $crate::interfaces::event_target::EventTarget;
                self.on(callback)
            }
        }

        impl $crate::interfaces::event_target::EventTarget<[<$event_type:camel>]> for [<$tag_name:camel Builder>] {}
    }
}}

#[doc(hidden)]
/// Define an element type.
#[macro_export]
macro_rules! element {
    (
        $(#[$outer:meta])*
        <$name:ident($text_name:expr)>
        $(categories { $($category:ident),+ })?
        $(children {
            $(tags { $(< $child_tag:ident >),+ })?
            $(categories { $($child_category:ident),+ })?
        })?
        $(attributes {$(
            $(#[$attr_meta:meta])*
            $attr:ident $(- $attr_tail:ident)* $(( $attr_ty:ty ))?
        )*})?
        $(custom_events {$(
            $(#[$event_meta:meta])*
            $event_type:ident $(- $event_type_tail:ident)*
        )*})?
    ) => { $crate::macros::__private::paste::item! {

        $(#[$outer])*
        ///
        /// A function for creating a builder which will accept attributes and produce the element.
        #[topo::nested]
        pub fn $name() -> [<$name:camel Builder>] {
            [<$name:camel Builder>] { inner: $crate::macros::__private::cache_elem($text_name) }
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
                $crate::macros::__private::remove_trailing_children(&self);

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
                    $crate::macros::__private::just_all_of_it_ok::[<$child_tag:camel>]>
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
                pub $attr $(- $attr_tail)* $(($attr_ty))?
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

        // custom events
        $($(
            $crate::custom_event!(
                $name {
                    $(#[$event_meta])*
                    [<$event_type $(_ $event_type_tail)*>] (
                        $crate::name_as_str!($event_type $(- $event_type_tail)*)
                    )
                }
            );
        )*)?
    }};
}

// TODO: Much more detailed docs
/// Define an HTML element type, which is essentially an `element!` with the
/// `HtmlElementBuilder` and `GlobalEventHandler` traits.
#[macro_export]
macro_rules! html_element {
    (
        $(#[$outer:meta])*
        <$name:ident $(- $name_tail:ident)*>
        $($rem:tt)*
    ) => {$crate::macros::__private::paste::item! {
        html_element!(
            $(#[$outer])*
            < [<$name $(_ $name_tail)*>] (
                $crate::name_as_str!($name $(- $name_tail)*)
            )>
            $($rem)*
        );
    }};
    (
        $(#[$outer:meta])*
        <$name:ident($text_name:expr)>
        $($rem:tt)*
    ) => { $crate::macros::__private::paste::item! {
        $crate::element! {
            $(#[$outer])*
            <$name($text_name)>
            $($rem)*
        }

        impl $crate::interfaces::html_element::HtmlElementBuilder for [<$name:camel Builder>] {}
        impl $crate::interfaces::global_events::GlobalEventHandler for [<$name:camel Builder>] {}

        impl<E> $crate::interfaces::event_target::EventTarget<E> for [<$name:camel Builder>]
        where E: $crate::interfaces::global_events::GlobalEvent {}
    }};
}

macro_rules! only_text_children {
    (<$name:ident>) => {
        paste::item! {
            impl crate::interfaces::node::Parent<crate::text::Text>
            for [<$name:camel Builder>] {}
        }
    };
}
