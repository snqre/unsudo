use super::*;

pub fn element(identifier: &str) -> Option<web_sys::Element> {
    document()?.get_element_by_id(identifier)
}

pub fn document() -> Option<web_sys::Document> {
    web_sys::window()?.document()
}