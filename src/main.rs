use std::{collections::HashMap, fmt::format};
use warp::{http::StatusCode, Filter};

async fn hello_world() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello, World!"))
}

async fn hello(param: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello {}!", param))
}

async fn hello_query(param: HashMap<String, String>) -> Result<impl warp::Reply, warp::Rejection> {
    let name = param.get("name");
    match name {
        Some(name) => Ok(format!("Hello, {}!", name)),
        None => Ok(format!("Hello, World! (Name is unknown)")),
    }
}

#[tokio::main]
async fn main() {
    // GET /hello => 200 OK with body "Hello, World!"
    let hello_world = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::end())
        .and_then(hello_world);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(hello);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello_query = warp::get()
        .and(warp::path("hello"))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(hello_query);

    // Serve routes
    let routes = hello_query.or(hello_world).or(hello);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
