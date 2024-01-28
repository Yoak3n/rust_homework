
// use crate::models::query::QueryModel;
use sqlx::postgres::PgPool;


pub struct AppState{
    // pub info: QueryModel,
    pub db: PgPool,
}