table! {
    clients (id) {
        id -> Nullable<Int4>,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Nullable<Int4>,
        email -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    clients,
    users,
);
