// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
        author -> Text,
        status -> Nullable<Text>,
        language -> Text,
    }
}
