use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::routes::game::{create_game, delete_game, get_game_by_id, get_games, update_game};

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
