use sqlx::{sqlite::{SqliteConnection, SqlitePoolOptions}, Pool, Sqlite, FromRow};


#[derive(Default)]
pub struct AppData {
    pub db_path: String,
    pub pool: Option<Pool<Sqlite>>,
}