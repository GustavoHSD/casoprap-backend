use poem::{error::{InternalServerError, Error}, handler, web::{Data, Json, Path}};
use sqlx::MySqlPool;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Animal {
    id: i32,
    name: String,
    race: String,
    a_type: String,
    age: Option<i32>,
    rescue_location: String,
    is_adopted: i8,
    responsible_volunteer: i32,
}

#[derive(Serialize, Deserialize)]
struct AnimalReq {
    name: String,
    race: String,
    a_type: String,
    age: i16,
    rescue_location: String,
    is_adopted: bool,
    responsible_volunteer: i32,
}

#[handler]
pub async fn create( 
    pool: Data<&MySqlPool>,
    req: Json<AnimalReq>,
    ) -> Json<serde_json::Value> {
    let id = sqlx::query!(
        "INSERT INTO Animal (name, race, a_type, age, rescue_location, responsible_volunteer) values (?,?,?,?,?,?)",
        req.name,
        req.race,
        req.a_type,
        req.age,
        req.rescue_location,
        req.responsible_volunteer
    )
    .execute(pool.0)
    .await
    .map_err(InternalServerError).expect("Could not insert animal")
    .last_insert_id();
    
    Json(serde_json::json!({
        "id": id,
        "name": req.name,
    }))
}

#[handler]
pub async fn find_all(pool: Data<&MySqlPool>,) -> Result<Json<Vec<Animal>>, Error> {
    let animals = sqlx::query_as!(
        Animal,
        "SELECT * FROM Animal"
    )
    .fetch_all(pool.0)
    .await
    .unwrap();
    
    Ok(Json(animals))
}

#[handler]
pub async fn find_by_id(Path(id): Path<i64>, pool: Data<&MySqlPool>,) -> Result<Json<Animal>, Error> {
    let animal = sqlx::query_as!(
        Animal,
        "SELECT * FROM Animal WHERE id = ?", 
        id as i64
    )
    .fetch_one(pool.0)
    .await
    .unwrap();
    
    Ok(Json(animal)) 
}
