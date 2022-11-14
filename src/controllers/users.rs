use diesel::{prelude::*};

use crate::models::{self, NewUser, User};
use uuid::Uuid;
/// Тип ошибок, возникающих при работе с базой данных
type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::schema::users::dsl::*;


/// Метод, возвращающий вектор объектов пользователей
/// # Arguments
///
/// * `conn`        - указатель на подключение к базе данных.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо вектор объектов пользователей.
pub fn get_users(conn: &PgConnection) -> Result<Option<Vec<models::User>>, DbError>{
    let users_list = users.load(conn).optional()?;

    Ok(users_list)
}

/// Метод, возвращающий пользователя по идентификатору
/// # Arguments
///
/// * `conn`        - указатель на подключение к базе данных.
/// * `uuid`        - уникальный идентификатор объекта пользователя.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо объект пользователя.
pub fn get_user(uuid: &Uuid, conn: &PgConnection) -> Result<Option<User>, DbError>{
    let user = users
    .filter(id.eq(uuid.to_string()))
    .first::<User>(conn)
    .optional()?;

    Ok(user)
}

/// Метод, создающий пользователя
/// # Arguments
///
/// * `conn`             - указатель на подключение к базе данных.
/// * `new_user`         - указатель на десериализованный объект структуры NewUser.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо объект пользователя.
pub fn create_user(new_user: &NewUser, conn: &PgConnection) -> Result<models::User, DbError>{
    let new = User{
        id: Uuid::new_v4().to_string(),
        user_name: new_user.user_name.clone(),
        password: new_user.password.clone(),
        email: new_user.email.clone(),
        role: new_user.role,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: None
    };
    diesel::insert_into(users).values(&new).execute(conn)?;
    Ok(new)
}

/// Метод, удаляющий пользователя по идентификатору
/// # Arguments
///
/// * `conn`        - указатель на подключение к базе данных.
/// * `uuid`        - уникальный идентификатор объекта пользователя.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, флаг, реализованный через тип bool.
pub fn delete_user(uuid: &Uuid, conn: &PgConnection) -> Result<bool, DbError>{
    let result = diesel::delete(users.filter(id.eq(uuid.to_string())))
        .execute(conn).is_ok();
    Ok(result)
}

/// Метод, изменяющий пользователя по идентификатору
/// # Arguments
///
/// * `conn`            - указатель на подключение к базе данных.
/// * `uuid`            - уникальный идентификатор объекта пользователя.
/// * `new_user`        - указатель на десериализованный объект структуры NewUser.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо измененный объект пользователя.
pub fn update_user(uuid: &Uuid, new_user: &NewUser, conn: &PgConnection) -> Result<User, DbError>{
    let user: User = diesel::update(users.filter(id.eq(uuid.to_string())))
        .set((
            user_name.eq(new_user.user_name.clone()), 
            password.eq(new_user.password.clone()),
            email.eq(new_user.email.clone()),
            role.eq(new_user.role),
            updated_at.eq(super::get_date()) 
        )).get_result(conn)?;
    Ok(user)
}