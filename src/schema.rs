// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        short_url -> Text,
        long_url -> Text,
    }
}
