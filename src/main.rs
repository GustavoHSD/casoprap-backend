use dotenv::dotenv;
use poem::{get, http::Method, listener::TcpListener, middleware::Cors, post, EndpointExt, Result, Route, Server};
use sqlx::MySqlPool;
use std::{env};

mod api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

    let cors = Cors::new()
        .allow_method(Method::GET)
        .allow_method(Method::POST)
        .allow_credentials(false)
        .allow_origin("http://localhost:5173");

    let app = Route::new()
        .at("/post/volunteer", post(api::volunteer::create))
        .at("/get/volunteer", get(api::volunteer::find_all))
        .at("get/volunteer/search/find_by_id/:id", get(api::volunteer::find_by_id))

        .at("/post/animal", post(api::animal::create))
        .at("/get/animal", get(api::animal::find_all))
        .at("/get/animal/search/find_by_id/:id", get(api::animal::find_by_id))

        .at("/post/resource", post(api::resource::create))
        .at("/get/resource", get(api::resource::find_all))
        .at("/get/resource/search/find_by_id/:id", get(api::resource::find_by_id))
        .with(cors)
        .data(pool);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app).await?;
    return Ok(())
}
