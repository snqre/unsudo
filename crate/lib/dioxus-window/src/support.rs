use super::*;

fn element(identifier: &str) -> Result<web_sys::Element> {
    document()?
        .get_element_by_id(identifier)
        .ok_or(Error::ElementIdNotFound)
}

fn document() -> Result<web_sys::Document> {
    web_sys::window()
        .ok_or(Error::WindowUnavailable)?
        .document()
        .ok_or(Error::DocumentUnavailable)
}