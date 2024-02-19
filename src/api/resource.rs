use poem::{error::{Error, InternalServerError}, handler, web::{Data, Json, Path}};
use sqlx::{FromRow, MySqlPool};
use serde::{Serialize, Deserialize};

#[derive(Serialize, FromRow)]
struct Resource {
    id: i32,
    description: String,
    price: f32,
    volunteer_id: i32
}

#[derive(Serialize, Deserialize)]
struct ResourceReq {
    description: String,
    price: f32,
    volunteer_id: i32
}

#[handler]
pub async fn create( 
    pool: Data<&MySqlPool>,
    req: Json<ResourceReq>,
    ) -> Json<serde_json::Value> {
    let id = sqlx::query!(
        "INSERT INTO Resource (description, price, volunteer_id) values (?,?,?)",
        req.description,
        req.price,
        req.volunteer_id
    )
    .execute(pool.0)
    .await
    .map_err(InternalServerError).expect("Could not insert volunteer")
    .last_insert_id();
    
    Json(serde_json::json!({
        "id": id,
    }))
}

#[handler]
pub async fn find_all(pool: Data<&MySqlPool>,) -> Result<Json<Vec<Resource>>, Error> {
    let resources = sqlx::query_as!(
        Resource,
        "SELECT * FROM Resource"
    )
    .fetch_all(pool.0)
    .await
    .unwrap();
    
    Ok(Json(resources))
}

#[handler]
pub async fn find_by_id(Path(id): Path<i64>, pool: Data<&MySqlPool>,) -> Result<Json<Resource>, Error> {
    let resource = sqlx::query_as!(
        Resource,
        "SELECT * FROM Resource WHERE id = ?",
        id
    )
    .fetch_one(pool.0)
    .await
    .unwrap();

    Ok(Json(resource)) 
}
