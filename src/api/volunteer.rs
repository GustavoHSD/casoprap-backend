use poem::{handler, http::StatusCode, web::{Data, Json, Path}, IntoResponse, Response};
use sqlx::MySqlPool;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Volunteer {
    id: i32,
    name: String,
    cpf: String,
    is_active: i8,
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
    ) -> impl IntoResponse {
    let query_result = sqlx::query!(
        "INSERT INTO Volunteer (name, cpf, is_active) values (?,?,?)",
        req.name,
        req.cpf,
        true
    )
    .execute(pool.0)
    .await;
     
    let response = match query_result {
        Ok(query_result) => {
            Json(serde_json::json!({
                "id": query_result.last_insert_id(),
                "name": req.name,
            })).into_response() 
        },
        Err(_) => { 
            let error_message = serde_json::json!({"error": "Volunteer could not be inserted"}); 
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response()
        }, 
    };
    response
}

#[handler]
pub async fn find_all(pool: Data<&MySqlPool>,) -> impl IntoResponse {
    let volunteers = sqlx::query_as!(
        Volunteer,
        "SELECT * FROM Volunteer"
    )
    .fetch_all(pool.0)
    .await;  

    let response = match volunteers {   
        Ok(volunteers) => Json(volunteers).into_response(),
        Err(error) => {
            let error_message = serde_json::json!({"error": error.to_string()}); 
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response()
        },
    }; 
    response
}

#[handler]
pub async fn find_by_id(Path(id): Path<i64>, pool: Data<&MySqlPool>) -> impl IntoResponse {
    let volunteer = sqlx::query_as!(
        Volunteer,
        "SELECT * FROM Volunteer WHERE id = ?",
        id
    )
    .fetch_one(pool.0)
    .await;

    let response = match volunteer {
        Ok(volunteer) => Json(volunteer).into_response(),
        Err(_) => { 
            let error_message = serde_json::json!({"message": "Volunteer not found"});
            Response::builder()
            .content_type("application/json; charset=utf-8")
            .status(StatusCode::NOT_FOUND)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response() 
        }
    };
    response
}


