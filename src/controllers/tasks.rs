
use diesel::{prelude::*};

use crate::models::{self, NewTask, Task};
use uuid::Uuid;
/// Тип ошибок, возникающих при работе с базой данных
type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::schema::tasks::dsl::*;

/// Метод, возвращающий вектор объектов задач
/// # Arguments
///
/// * `conn`        - указатель на подключение к базе данных.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо вектор объектов задач.
pub fn get_tasks(conn: &PgConnection) -> Result<Option<Vec<models::Task>>, DbError>{
    let tasks_list = tasks.load(conn).optional()?;

    Ok(tasks_list)
}

/// Метод, возвращающий задачу по идентификатору
/// # Arguments
///
/// * `conn`        - указатель на подключение к базе данных.
/// * `uuid`        - уникальный идентификатор объекта задачи.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо объект задачи.
pub fn get_task(uuid: &Uuid, conn: &PgConnection) -> Result<Option<Task>, DbError>{
    let task = tasks
    .filter(id.eq(uuid.to_string()))
    .first::<Task>(conn)
    .optional()?;

    Ok(task)
}

/// Метод, создающий задачу
/// # Arguments
///
/// * `conn`             - указатель на подключение к базе данных.
/// * `new_task`         - указатель на десериализованный объект структуры NewTask.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо объект задачи.
pub fn create_task(new_task: &NewTask, conn: &PgConnection) -> Result<models::Task, DbError>{
    let new = Task{
        id: Uuid::new_v4().to_string(),
        title: new_task.title.clone(),
        body: new_task.body.clone(),
        done: false,
        user_id: new_task.user_id.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: None
    };
    diesel::insert_into(tasks).values(&new).execute(conn)?;
    Ok(new)
}

/// Метод, удаляющий задачу по идентификатору
/// # Arguments
///
/// * `conn`        - указатель на подключение к базе данных.
/// * `uuid`        - уникальный идентификатор объекта задачи.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, флаг, реализованный через тип bool.
pub fn delete_task(uuid: &Uuid, conn: &PgConnection) -> Result<bool, DbError>{
    let old_count = tasks.count().first::<i64>(conn)?;
    diesel::delete(tasks.filter(id.eq(uuid.to_string())))
        .execute(conn)?;
    let new_count = tasks.count().first::<i64>(conn)?;
    let mut result = false;
    if old_count > new_count{
        result = true;
    }
    Ok(result)
}

/// Метод, изменяющий задачу по идентификатору
/// # Arguments
///
/// * `conn`            - указатель на подключение к базе данных.
/// * `uuid`            - уникальный идентификатор объекта задачи.
/// * `new_task`        - указатель на десериализованный объект структуры NewTask.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо измененный объект задачи.
pub fn update_task(uuid: &Uuid, new_task: &NewTask, conn: &PgConnection) -> Result<Task, DbError>{
    let task: Task = diesel::update(tasks.filter(id.eq(uuid.to_string())))
        .set((
            title.eq(new_task.title.clone()),
            body.eq(new_task.body.clone()),
            done.eq(new_task.done),
            user_id.eq(new_task.user_id.clone()),
            updated_at.eq(super::get_date()) 
        )).get_result(conn)?;
    Ok(task)
}