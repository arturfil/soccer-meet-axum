use std::sync::Arc;
use axum::{extract::{State, Query, Path}, Json, http::StatusCode, response::IntoResponse};
use crate::{AppState, models::{dtos::{FilterOptions, CreateGameSchema, UpdateGameSchema}, game::GameModel}};

pub async fn get_games_svc(
    opts: Option<Query<FilterOptions>>, 
    data: State<Arc<AppState>>) -> Result<Vec<GameModel>, sqlx::Error> {
    
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    sqlx::query_as!(
        GameModel,
        "SELECT id, day, field_name, address, created_at, updated_at FROM games LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db).await
}

pub async fn create_game_service( 
    State(data): State<Arc<AppState>>, 
    Json(body): Json<CreateGameSchema>) -> Result<GameModel, sqlx::Error> { 
    
    sqlx::query_as!(
        GameModel,
        "INSERT INTO games (field_name, address, day) 
        VALUES ($1, $2, $3) RETURNING id, day, field_name, address, created_at, updated_at",
        body.field_name.to_string(),
        body.address.to_string(),
        body.day.to_string()
    )
    .fetch_one(&data.db).await
}

pub async fn get_game_by_id_service(
    Path(id): Path<uuid::Uuid>, 
    State(data): State<&Arc<AppState>>
) -> Result<GameModel, sqlx::Error> {

    sqlx::query_as!(
        GameModel,
        "SELECT id, day, field_name, address, created_at, updated_at 
        FROM games WHERE id = $1",
        id
    ).fetch_one(&data.db).await
}

pub async fn update_game_service(
    Path(id): Path<uuid::Uuid>,
    State(data): State<&Arc<AppState>>,
    Json(body): Json<UpdateGameSchema>,
    game: GameModel
) -> Result<GameModel, sqlx::Error> {

    let now = chrono::Utc::now();

    sqlx::query_as!(
        GameModel,
        "UPDATE games set field_name = $1, address = $2, day = $3, updated_at = $4 
        WHERE id = $5 RETURNING id, day, field_name, address, created_at, updated_at",
        body.field_name.to_owned().unwrap_or(game.field_name),
        body.address.to_owned().unwrap_or(game.address),
        body.day.to_owned().unwrap_or(game.day),
        now,
        id 
    ).fetch_one(&data.db).await
}

pub async fn delete_game_service(
    Path(id): Path<uuid::Uuid>,
    State(data): State<&Arc<AppState>> 
) -> sqlx::query::Query<sqlx::Postgres, sqlx::postgres::PgArguments> {
    
    sqlx::query!("DELETE FROM games WHERE id = $1", id)
}


