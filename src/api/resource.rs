use poem::{handler, http::StatusCode, web::{Data, Json, Path}, IntoResponse, Response};
use sqlx::MySqlPool;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
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
    ) -> impl IntoResponse {
    let query_result = sqlx::query!(
        "INSERT INTO Resource (description, price, volunteer_id) values (?,?,?)",
        req.description,
        req.price,
        req.volunteer_id
    )
    .execute(pool.0)
    .await;
     
    let response = match query_result {
        Ok(query_result) => {
            Json(serde_json::json!({
                "id": query_result.last_insert_id(),
            })).into_response() 
        },
        Err(_) => { 
            let error_message = serde_json::json!({"error": "Resource could not be inserted"}); 
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
    let resources = sqlx::query_as!(
        Resource,
        "SELECT * FROM Resource"
    )
    .fetch_all(pool.0)
    .await;  

    let response = match resources {   
        Ok(resources) => Json(resources).into_response(),
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
pub async fn find_by_id(Path(id): Path<i64>, pool: Data<&MySqlPool>,) -> impl IntoResponse {
    let resource = sqlx::query_as!(
        Resource,
        "SELECT * FROM Resource WHERE id = ?",
        id
    )
    .fetch_one(pool.0)
    .await;  

    let response = match resource {   
        Ok(resource) => Json(resource).into_response(),
        Err(_) => {
            let error_message = serde_json::json!({"message": "Resource not found"});
            Response::builder()
            .content_type("application/json; charset=utf-8")
            .status(StatusCode::NOT_FOUND)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response() 
         },
    }; 
    response
}

#[handler]
pub async fn find_by_volunteer(Path(volunteer_id): Path<i64>, pool: Data<&MySqlPool>,) -> impl IntoResponse {
    let resources = sqlx::query_as!(
        Resource,
        "SELECT * FROM Resource WHERE volunteer_id = ?",
        volunteer_id
    )
    .fetch_all(pool.0)
    .await;  

    let response = match resources {   
        Ok(resources) => Json(resources).into_response(),
        Err(_) => {
            let error_message = serde_json::json!({"message": "Volunteer has no resources associated"});
            Response::builder()
            .content_type("application/json; charset=utf-8")
            .status(StatusCode::NOT_FOUND)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response() 
         },
    }; 
    response
}
