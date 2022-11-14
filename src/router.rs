use crate::{database::DbPool, models::NewTask, models::NewUser};
use actix_web::{HttpResponse, Error, web, get, post, delete, put};
use crate::controllers;
use uuid::Uuid;

/// Метод, обрабатывающий GET запрос.
/// # Arguments
///
/// * `pool` - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо вектор заданий.

#[get("/")]
async fn get_tasks(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let tasks = web::block(move ||{
        let conn = pool.get()?;
        controllers::tasks::get_tasks(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(tasks) = tasks {
        Ok(HttpResponse::Ok().json(tasks))
    } else {
        let res = HttpResponse::NotFound().body("No tasks found");
        Ok(res)
    }
}

/// Метод, обрабатывающий GET запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `task_uid`    - Уникальный идентификатор задачи, требуемой для извлечения из базы данных.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, объект задачи.

#[get("/task/{task_uid}")]
async fn get_task(pool: web::Data<DbPool>, task_uid: web::Path<Uuid>) -> Result<HttpResponse, Error>{
    log::info!("test");
    let task_uid = task_uid.into_inner();
    let task = web::block(move || {
        let conn = pool.get()?;
        controllers::tasks::get_task(&task_uid, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(task) = task{
        Ok(HttpResponse::Ok().json(task))
    } else {
        let response = HttpResponse::NotFound().body(format!("Task {} not found", task_uid));
        Ok(response)
    }
}

/// Метод, обрабатывающий POST запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `new_task`    - Структура данных типа new_task, необходимая для создания объекта сущности task.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, объект задачи.

#[post("/task")]
async fn add_task(pool: web::Data<DbPool>, new_task: web::Json<NewTask>) -> Result<HttpResponse, Error>{
    let task = web::block(move || {
        let conn = pool.get()?;
        controllers::tasks::create_task(&new_task.0, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

/// Метод, обрабатывающий DELETE запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `task_uid`    - Уникальный идентификатор задачи, требуемой для удаления из базы данных.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо сообщение об успешном удалении объекта сущности задачи.

#[delete("/task/{task_uid}")]
async fn delete_task(pool: web::Data<DbPool>, task_uid: web::Path<Uuid>)-> Result<HttpResponse, Error>{
    let task_uid = task_uid.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        controllers::tasks::delete_task(&task_uid, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if result == true{
        Ok(HttpResponse::Ok().body(format!("Task {} deleted", task_uid.to_string())))
    } else {
        Ok(HttpResponse::NotFound().body(format!("Task {} not found", task_uid)))
    }
    
}

/// Метод, обрабатывающий PUT запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `task_uid`    - Уникальный идентификатор задачи, требуемой для извлечения из базы данных.
/// * `new_task`    - Структура данных типа new_task, необходимая для создания объекта сущности task.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, объект задачи.

#[put("/task/{task_uid}")]
async fn update_task(
    pool: web::Data<DbPool>,
    new_task: web::Json<NewTask>,
    task_uid: web::Path<Uuid>
)-> Result<HttpResponse, Error>{
    let task_uid = task_uid.into_inner();
    let task = web::block(move || {
        let conn = pool.get()?;
        controllers::tasks::update_task(&task_uid,&new_task.0, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(task))
}

/// Метод, обрабатывающий GET запрос.
/// # Arguments
///
/// * `pool` - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо вектор пользователей.

#[get("/users")]
async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let users = web::block(move ||{
        let conn = pool.get()?;
        controllers::users::get_users(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(users) = users {
        Ok(HttpResponse::Ok().json(users))
    } else {
        let res = HttpResponse::NotFound().body("No tasks found");
        Ok(res)
    }
}

/// Метод, обрабатывающий GET запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `user_uid`    - Уникальный идентификатор пользователя, требуемой для извлечения из базы данных.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, объект пользователя.

#[get("/user/{user_uid}")]
async fn get_user(pool: web::Data<DbPool>, user_uid: web::Path<Uuid>) -> Result<HttpResponse, Error>{
    let user_uid = user_uid.into_inner();
    let user = web::block(move || {
        let conn = pool.get()?;
        controllers::users::get_user(&user_uid, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user{
        Ok(HttpResponse::Ok().json(user))
    } else {
        let response = HttpResponse::NotFound().body(format!("User {} not found", user_uid));
        Ok(response)
    }
}

/// Метод, обрабатывающий POST запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `new_user`    - Структура данных типа new_user, необходимая для создания объекта сущности пользователя.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, объект пользователя.

#[post("/user")]
async fn add_user(pool: web::Data<DbPool>, new_user: web::Json<NewUser>) -> Result<HttpResponse, Error>{
    let task = web::block(move || {
        let conn = pool.get()?;
        controllers::users::create_user(&new_user.0, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(task))
}

/// Метод, обрабатывающий DELETE запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `user_uid`    - Уникальный идентификатор пользователя, требуемой для удаления из базы данных.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, либо сообщение об успешном удалении объекта сущности пользователя.

#[delete("/user/{user_uid}")]
async fn delete_user(pool: web::Data<DbPool>, user_uid: web::Path<Uuid>)-> Result<HttpResponse, Error>{
    let user_uid = user_uid.into_inner();
    let result = web::block(move || {
        let conn = pool.get()?;
        controllers::users::delete_user(&user_uid, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if result{
        Ok(HttpResponse::Ok().body(format!("User {} deleted", user_uid.to_string())))
    } else {
        Ok(HttpResponse::NotFound().body(format!("User {} not found", user_uid)))
    }
}

/// Метод, обрабатывающий PUT запрос.
/// # Arguments
///
/// * `pool`        - Пул базы данных. Данный аргумент обрабатывается фреймворком Actix.
/// * `user_uid`    - Уникальный идентификатор пользователя, требуемой для извлечения из базы данных.
/// * `new_user`    - Структура данных типа new_user, необходимая для создания объекта сущности пользователя.
///
/// # Return
///
/// Возвращает Результат с ответом, содержащим либо ошибку, объект пользователя.

#[put("/user/{user_uid}")]
async fn update_user(
    pool: web::Data<DbPool>,
    new_user: web::Json<NewUser>,
    user_uid: web::Path<Uuid>
)-> Result<HttpResponse, Error>{
    let user_uid = user_uid.into_inner();
    let user = web::block(move || {
        let conn = pool.get()?;
        controllers::users::update_user(&user_uid,&new_user.0, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user))
}