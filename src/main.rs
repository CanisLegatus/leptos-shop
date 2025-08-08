use tower_http::services::ServeDir;
use axum::Router;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_config::get_configuration;

mod app;


#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    
    let leptos_options = conf.leptos_options;


}
