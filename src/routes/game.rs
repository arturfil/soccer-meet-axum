use std::sync::Arc;
use axum::{response::IntoResponse, extract::{State, Query, Path}, http::StatusCode, Json};
use serde_json::json;
use crate::{models::{dtos::{FilterOptions, CreateGameSchema, UpdateGameSchema}, game::GameModel}, AppState};


pub async fn get_games(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        GameModel,
        "SELECT id, day, field_name, address, created_at, updated_at FROM games LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
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
    Json(body): Json<CreateGameSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        "INSERT INTO games (field_name, address, day) VALUES ($1, $2, $3) RETURNING id, day, field_name, address, created_at, updated_at",
        body.field_name.to_string(),
        body.address.to_string(),
        body.day.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
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

pub async fn get_game_by_id(Path(id): Path<uuid::Uuid>, State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        GameModel,
        "SELECT id, day, field_name, address, created_at, updated_at FROM games WHERE id = $1",
        id
    )
        .fetch_one(&data.db)
        .await;

    match query_result {
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
    let query_result = sqlx::query_as!(
        GameModel,
        "SELECT id, day, field_name, address, created_at, updated_at FROM games WHERE id = $1",
        id
    )
        .fetch_one(&data.db).await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let game = query_result.unwrap();

    let query_result = sqlx::query_as!(
        GameModel,
        "UPDATE games set field_name = $1, address = $2, day = $3, updated_at = $4 WHERE id = $5 RETURNING id, day, field_name, address, created_at, updated_at",
        body.field_name.to_owned().unwrap_or(game.field_name),
        body.address.to_owned().unwrap_or(game.address),
        body.day.to_owned().unwrap_or(game.day),
        now,
        id 
    )
        .fetch_one(&data.db).await;

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

pub async fn delete_game(Path(id): Path<uuid::Uuid>, State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM games WHERE id = $1", id)
        .execute(&data.db)
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
