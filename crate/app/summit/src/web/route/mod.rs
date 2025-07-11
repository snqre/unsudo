use super::*;
use landing::home::Route as Home;

mod app;
mod landing;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {}
}