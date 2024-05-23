// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        address -> Text,
        date_created -> Text,
    }
}
