use super::*;


#[component]
pub fn Navbar() -> Element {
    rsx!(
        NavbarScaffold {
            left: rsx!(
                decor::Logo {}
            )
        }
    )
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct NavbarScaffoldProps {
    pub left: Option<Element>,
    pub center: Option<Element>,
    pub right: Option<Element>
}

#[component]
pub fn NavbarScaffold(props: NavbarScaffoldProps) -> Element {
    rsx!(
        dioxus_layout::Row {
            dioxus_layout::Row {}
            dioxus_layout::Row {}
            dioxus_layout::Row {}
        }
    )
}