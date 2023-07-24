// @generated automatically by Diesel CLI.

diesel::table! {
    signatures (id) {
        id -> Int4,
        signature -> Varchar,
        bytes -> Varchar,
        abi -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
