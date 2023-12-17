use axum::extract::{Query, Path};
use tokio::net::TcpListener;
use serde_derive::Deserialize;
use axum::{Router, middleware};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());


    let addr = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING ON {:?}", addr);
    axum::serve(addr, routes_all).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    
    println!();
    res
} 

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
} 

fn routes_hello() -> Router {
    Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// Dans une query genre hello?name="Jen"
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");
    
    Html(format!("Hello <strong>{name}</strong>"))
}