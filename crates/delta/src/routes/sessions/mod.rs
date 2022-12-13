use rocket::Route;
use rocket_okapi::okapi::openapi3::OpenApi;

mod edit;
mod fetch_all;
mod login;
mod logout;
mod revoke;
mod revoke_all;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        login::req,
        logout::req,
        edit::req,
        fetch_all::req,
        revoke::req,
        revoke_all::req
    ]
}
