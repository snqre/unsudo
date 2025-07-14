/// `test!` is a *convenience* macro for bootstrapping a minimal Dioxus app for 
/// *component development*, *previews*, or *demos*. It sets up routing between multiple components, and
/// wraps them inside a styled container for isolated testing.
/// # Dependency
/// * `dioxus`
/// # Example
/// ###### `src/main.rs`
/// ```rust
/// test!(
///     style: "asset/test-container.css";
///     "/button" => Button
///     "/form" => Form
/// );
/// ```
/// ###### `asset/test-container.css`
/// ```css
/// * {
///     box-sizing: border-box;
///     padding: 0;
///     margin: 0;
/// }
/// 
/// #test {
///     display: flex;
///     flex-direction: column;
///     justify-content: center;
///     align-items: center;
///     width: 100vw;
///     height: 100vh;
///     background: #ffffff;
/// }
/// ```
#[macro_export]
macro_rules! test {
    (# style: $style:literal; $($path:expr => $route:ident)*) => {
        use dioxus::prelude::*;
        use dioxus::document;

        #[derive(Routable)]
        #[derive(Clone)]
        #[derive(PartialEq)]
        pub enum Route {
            $(
                #[route($path)]
                $route {},
            )*
        }

        #[component]
        fn Main() -> Element {
            rsx!(
                document::Title { "Test" }
                document::Stylesheet { href: asset!($style) }
                div { id: "test.css", Router::<Route> {} }
            )
        }

        fn main() {
            launch(Main);
        }
    };
}