// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        checked_in_time -> Text,
        table_number -> Int4,
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

diesel::table! {
    orders (id) {
        id -> Int4,
        published_at -> Text,
        quantity -> Int4,
        item_id -> Int4,
        customer_id -> Int4,
    }
}

diesel::joinable!(orders -> customers (customer_id));
diesel::joinable!(orders -> items (item_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    items,
    orders,
);
