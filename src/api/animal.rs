use poem::{handler, http::StatusCode, web::{Data, Json, Path}, IntoResponse, Response};
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
    responsible_volunteer: i32, }

#[handler]
pub async fn create( 
    pool: Data<&MySqlPool>,
    req: Json<AnimalReq>,
    ) -> impl IntoResponse {
    let query_result = sqlx::query!(
        "INSERT INTO Animal (name, race, a_type, age, rescue_location, responsible_volunteer) values (?,?,?,?,?,?)",
        req.name,
        req.race,
        req.a_type,
        req.age,
        req.rescue_location,
        req.responsible_volunteer
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
            let error_message = serde_json::json!({"error": "Animal could not be inserted"}); 
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
    let animals = sqlx::query_as!(
        Animal,
        "SELECT * FROM Animal"
    )
    .fetch_all(pool.0)
    .await;  

    let response = match animals {   
        Ok(animals) => Json(animals).into_response(),
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
    let animal = sqlx::query_as!(
        Animal,
        "SELECT * FROM Animal WHERE id = ?", 
        id
    )
    .fetch_one(pool.0)
    .await;

    let response = match animal {   
        Ok(animals) => Json(animals).into_response(),
        Err(_) => {
            let error_message = serde_json::json!({"error": "Animal not found"}); 
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response()
        },
    }; 
    response
}

#[handler]
pub async fn find_by_volunteer(Path(volunteer_id): Path<i64>, pool: Data<&MySqlPool>,) -> impl IntoResponse {
    let animals = sqlx::query_as!(
        Animal,
        "SELECT * FROM Animal WHERE responsible_volunteer = ?",
        volunteer_id
    )
    .fetch_all(pool.0)
    .await; 

    let response = match animals {   
        Ok(animals) => Json(animals).into_response(),
        Err(_) => {
            let error_message = serde_json::json!({"message": "Volunteer has no animals associated"});
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
pub async fn set_as_adopted(Path(id): Path<i64>, pool: Data<&MySqlPool>,) -> impl IntoResponse {
    let query_result = sqlx::query_as!(
        Animal,
        "UPDATE Animal SET is_adopted=true WHERE id = ?",
        id
    )
    .execute(pool.0)
    .await;  

    let response = match query_result {   
        Ok(_) => Response::builder().status(StatusCode::OK).finish().into_response(),
        Err(_) => {
            let error_message = serde_json::json!({"message": "Volunteer has no animals associated"});
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
pub async fn set_as_not_adopted(Path(id): Path<i64>, pool: Data<&MySqlPool>,) -> impl IntoResponse {
    let query_result = sqlx::query_as!(
        Animal,
        "UPDATE Animal SET is_adopted=false WHERE id = ?",
        id
    )
    .execute(pool.0)
    .await;  

    let response = match query_result {   
        Ok(_) => Response::builder().status(StatusCode::OK).finish().into_response(),
        Err(_) => {
            let error_message = serde_json::json!({"message": "Volunteer has no animals associated"});
            Response::builder()
            .content_type("application/json; charset=utf-8")
            .status(StatusCode::NOT_FOUND)
            .body(serde_json::to_string(&error_message).unwrap())
            .into_response() 
         },
    }; 
    response
}
