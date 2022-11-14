use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

/// Публичный тип пула подключений к базе данныхы
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Метод инициализации пула базы данных. Подтягивает URL базы данных из .env файла
pub fn init_pool() -> DbPool{
    let db_address = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let conn_manager = ConnectionManager::<PgConnection>::new(db_address);
    let pool = r2d2::Pool::builder()
        .build(conn_manager)
        .expect("Failed to create pool");
    return pool;
}