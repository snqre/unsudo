use super::*;
mod home;

use home::Route as Home;

#[derive(Routable)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Route {
    #[route("/")]
    Home {}
}