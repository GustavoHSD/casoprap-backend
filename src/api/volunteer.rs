use poem::{error::{InternalServerError, Error}, handler, web::{Data, Json, Path}};
use sqlx::MySqlPool;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Volunteer {
    id: i32,
    name: String,
    cpf: String,
}

#[derive(Serialize, Deserialize)]
struct VolunteerReq {
    name: String,
    cpf: String,
}

#[handler]
pub async fn create( 
    pool: Data<&MySqlPool>,
    req: Json<VolunteerReq>,
    ) -> Json<serde_json::Value> {
    let id = sqlx::query!(
        "INSERT INTO Volunteer (name, cpf) values (?,?)",
        req.name,
        req.cpf
    )
    .execute(pool.0)
    .await
    .map_err(InternalServerError).expect("Could not insert volunteer")
    .last_insert_id();
    
    Json(serde_json::json!({
        "id": id,
        "name": req.name,
    }))
}

#[handler]
pub async fn find_all(pool: Data<&MySqlPool>,) -> Result<Json<Vec<Volunteer>>, Error> {
    let volunteers = sqlx::query_as!(
        Volunteer,
        "SELECT * FROM Volunteer"
    )
    .fetch_all(pool.0)
    .await
    .unwrap();
    
    Ok(Json(volunteers))
}

#[handler]
pub async fn find_by_id(Path(id): Path<i64>, pool: Data<&MySqlPool>,) -> Result<Json<Volunteer>, Error> {
    let volunteer = sqlx::query_as!(
        Volunteer,
        "SELECT * FROM Volunteer WHERE id = ?",
        id
    )
    .fetch_one(pool.0)
    .await
    .unwrap();
    
    Ok(Json(volunteer)) 
}
