// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;

    labels (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
        label_id -> Int4,
        is_checked -> Bool,
    }
}

diesel::joinable!(todos -> labels (label_id));

diesel::allow_tables_to_appear_in_same_query!(
    labels,
    todos,
);
