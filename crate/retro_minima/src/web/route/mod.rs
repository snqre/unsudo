use super::*;
mod home;
pub mod home_page;

use home_page::*;

#[derive(Routable)]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {}
}