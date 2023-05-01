use crate::{AppState, controllers::game::{get_games, get_game_by_id, update_game, delete_game, create_game}};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/games", get(get_games))
        .route(
            "/api/v1/games/game/:id",
            get(get_game_by_id).put(update_game).delete(delete_game),
        )
        .route("/api/v1/games/game", post(create_game))
        .with_state(app_state)
}
