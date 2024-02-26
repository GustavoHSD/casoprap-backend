use dotenv::dotenv;
use poem::{get, http::Method, listener::TcpListener, middleware::Cors, post, EndpointExt, Result, Route, Server};
use sqlx::MySqlPool;
use std::env;

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
        .at("/volunteer", post(api::volunteer::create).get(api::volunteer::find_all))
        .at("/volunteer/:id", get(api::volunteer::find_by_id))

        .at("/animal", post(api::animal::create).get(api::animal::find_all))
        .at("/animal/:id", get(api::animal::find_by_id))
        .at("/animal/by_volunteer/:volunteer_id", post(api::animal::find_by_volunteer))
        .at("/animal/:id/set_as_adopted", get(api::animal::set_as_adopted))
        .at("/animal/:id/set_as_not_adopted", get(api::animal::set_as_not_adopted))

        .at("/resource", post(api::resource::create).get(api::resource::find_all))
        .at("/resource/:id", get(api::resource::find_by_id))
        .at("/resource/by_volunteer/:volunteer_id", get(api::resource::find_by_volunteer))
        .with(cors)
        .data(pool);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app).await?;
    return Ok(())
}
