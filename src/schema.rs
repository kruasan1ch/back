
/// Макрос для работы с таблицей tasks
table! {
    tasks (id) {
        id -> Varchar,
        title -> Varchar,
        body -> Varchar,
        done -> Bool,
        user_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

/// Макрос для работы с таблицей users
table! {
    users (id) {
        id -> Varchar,
        user_name -> Varchar,
        password -> Varchar,
        email -> Varchar,
        role -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
