use sqlx::sqlite::SqlitePoolOptions;

pub enum DatabaseType {
    Sqlite,
}

impl Clone for DatabaseType {
    fn clone(&self) -> Self {
        match self {
            Sqlite => DatabaseType::Sqlite,
        }
    }
}

pub struct DatabaseConnection<T: sqlx::Database> {
    db_type: DatabaseType,
    pub pool: Option<sqlx::Pool<T>>,
}

impl<T: sqlx::Database> Clone for DatabaseConnection<T> {
    fn clone(&self) -> Self {
        DatabaseConnection {
            db_type: self.db_type.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl<T> DatabaseConnection<T>
where
    T: sqlx::Database,
{
    pub fn new(db_type: DatabaseType) -> DatabaseConnection<T> {
        DatabaseConnection {
            db_type,
            pool: None,
        }
    }

    pub fn clone_pool(&self) -> sqlx::Pool<T> {
        self.pool.clone().unwrap()
    }
}

impl DatabaseConnection<sqlx::Sqlite> {
    pub async fn connect(&mut self, uri: &str) -> Result<(), &'static str> {
        self.pool = match &self.db_type {
            DatabaseType::Sqlite => Some(
                SqlitePoolOptions::new()
                    .connect(uri)
                    .await
                    .expect("Error opening sqlite DB"),
            ),
        };

        Ok(())
    }
}
