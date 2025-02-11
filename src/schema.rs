// @generated automatically by Diesel CLI.
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
