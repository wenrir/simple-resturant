// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        checked_in_time -> Timestamp,
        total -> Int4,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        description -> Text,
        estimated_minutes -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    items,
);
