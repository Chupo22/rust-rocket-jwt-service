pub mod auth {
    table! {
        auth.refresh_token (id) {
            id -> Uuid,
            user_id -> Uuid,
            refresh_token -> Uuid,
        }
    }

    table! {
        auth.user (id) {
            id -> Uuid,
            first_name -> Varchar,
            last_name -> Varchar,
            patronymic -> Varchar,
            email -> Varchar,
            password -> Varchar,
        }
    }

    joinable!(refresh_token -> user (user_id));

    allow_tables_to_appear_in_same_query!(
        refresh_token,
        user,
    );
}
