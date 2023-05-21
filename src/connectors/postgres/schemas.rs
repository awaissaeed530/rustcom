// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}
