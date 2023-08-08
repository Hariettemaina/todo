// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Int4,
        todo_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    category_lookup (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        username -> Varchar,
        title -> Varchar,
        completed -> Bool,
        description -> Nullable<Varchar>,
        due_date -> Nullable<Date>,
        completed_date -> Nullable<Date>,
    }
}

diesel::table! {
    users (username) {
        username -> Varchar,
        display_name -> Nullable<Varchar>,
        password -> Varchar,
        email_address -> Varchar,
        email_verified -> Bool,
        email_verification_code -> Uuid,
        email_verification_code_expiry -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    category,
    category_lookup,
    todos,
    users,
);
