// @generated automatically by Diesel CLI.

diesel::table! {
    artistas (id) {
        id -> Int4,
        #[max_length = 255]
        nome -> Varchar,
    }
}

diesel::table! {
    imagemartista (id) {
        id -> Int4,
        iddoartista -> Nullable<Int4>,
        imagem -> Text,
    }
}

diesel::table! {
    todo (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        done -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        due_date -> Nullable<Timestamp>,
    }
}

diesel::joinable!(imagemartista -> artistas (iddoartista));

diesel::allow_tables_to_appear_in_same_query!(
    artistas,
    imagemartista,
    todo,
);
