use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres, PgPool};

use crate::models::game::GameModel;


#[derive(Clone)]
pub struct GameRepository {
    db: Arc<PgPool>
}

impl GameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            db: Arc::new(pool)
        }
    }

    pub fn db(&self) -> Arc<PgPool> {
        Arc::clone(&self.db)
    }

    pub async fn all(&self) -> Result<Vec<GameModel>, sqlx::Error> {
        sqlx::query_as(
            "SELECT * FROM games"
        ).fetch_all(self.db().as_ref()).await
    }
}


