use diesel::Insertable;
use serde::{Deserialize,Serialize};
use crate::schema::{tasks, users};
use chrono;

/// Модель сущности задания. Используется для работы ОРМ Diesel
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, Insertable)]
#[table_name = "tasks"]
#[belongs_to(User)]
pub struct Task{
    pub id: String,
    pub title: String,
    pub body: String, 
    pub done: bool,
    pub user_id: Option<String>, 
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>
}

/// Вспомогательная модель. 
/// Используется в качестве шаблона для десериализации данных, отправленных с бэкенда.
#[derive(Serialize,Deserialize)]
pub struct NewTask{
    pub title: String,
    pub body: String,
    pub done: bool,
    pub user_id: Option<String>,
}

/// Модель сущности пользователя. Используется для работы ОРМ Diesel
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "users"]
pub struct User{
    pub id: String,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub role: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>
}

/// Вспомогательная модель. 
/// Используется в качестве шаблона для десериализации данных, отправленных с бэкенда.
#[derive(Serialize,Deserialize)]
pub struct NewUser{
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub role: i32,
}