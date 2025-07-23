use super::*;

modwire::expose!(
    home
    logs
);

#[derive(Routable)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Route {
    #[route("/")]
    Home {}
}