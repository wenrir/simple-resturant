//! mod
pub(crate) mod order;

// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        description -> Text,
        quantity -> Int4,
        order_id -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        table_number -> Int4,
        published_at -> Timestamp,
    }
}

diesel::joinable!(items -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(items, orders,);
