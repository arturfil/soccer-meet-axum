use std::sync::Arc;
use axum::{response::IntoResponse, extract::{State, Query, Path}, http::StatusCode, Json};
use serde_json::json;
use crate::{models::{dtos::{FilterOptions, CreateGameSchema, UpdateGameSchema}, game::GameModel}, AppState, services::game::{get_games_svc, create_game_service, get_game_by_id_service, update_game_service, delete_game_service}}; 


pub async fn get_games(
        opts: Option<Query<FilterOptions>>,
        State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    // await for the respo service return with the sql query 
    let query_result = get_games_svc(opts, State(data)).await;
    if query_result.is_err() {  // error handling
        let err_response = serde_json::json!({
            "status": "fail",
            "message": "Something wrong happened.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_response)));
    }

    let games = query_result.unwrap();
    let json_response = serde_json::json!({
        "status": "success",
        "resutls": games.len(),
        "games": games
    });
    Ok(Json(json_response))
}

pub async fn create_game(
        State(data): State<Arc<AppState>>,
        Json(body): Json<CreateGameSchema>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let query_result = create_game_service(State(data), Json(body));

    match query_result.await {
        Ok(game) => {
            let game_response = json!({"status": "success", "data": json!({
                "game": game
            })});
            Ok ((StatusCode::CREATED, Json(game_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Game with that address already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            Err((StatusCode::INTERNAL_SERVER_ERROR, 
                 Json(json!({"status": "error", "message": format!("{:?}", e)}))
            ))
        }
    }
}

pub async fn get_game_by_id(
    Path(id): Path<uuid::Uuid>, 
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let query_result = get_game_by_id_service(Path(id), State(&data));
    
    match query_result.await {
        Ok(game) => {
            let game_response = serde_json::json!({
                    "status": "success",
                    "data": serde_json::json!({
                    "game": game
                })
            });
            return Ok(Json(game_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Game with ID: {} not foudn", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn update_game(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateGameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let query_result = get_game_by_id_service(Path(id), State(&data)).await;
    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    
    let game = query_result.unwrap();
    let query_result = update_game_service(Path(id), State(&data), Json(body), game).await;

    match query_result {
        Ok(game) => {
            let game_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "game": game
            })});
            return Ok(Json(game_response));
        }
        Err(err) => {
            return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"status": "error", "message": format!("{:?}", err)}))
            ))
        }
    }
}

pub async fn delete_game(
    Path(id): Path<uuid::Uuid>, 
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = delete_game_service(Path(id), State(&data))
        .await.execute(&data.db)
        .await.unwrap().rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    Ok(StatusCode::NO_CONTENT)
}
