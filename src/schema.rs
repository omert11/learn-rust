// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int2,
        #[max_length = 50]
        name -> Varchar,
        balance -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        customer_id -> Int2,
        before_balance -> Numeric,
        after_balance -> Numeric,
        amount -> Numeric,
        #[sql_name = "type"]
        #[max_length = 255]
        type_ -> Varchar,
        #[max_length = 255]
        description -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(transactions -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    transactions,
);
