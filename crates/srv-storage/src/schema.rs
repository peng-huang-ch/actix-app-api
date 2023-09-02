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

diesel::table! {
    tokens (id) {
        id -> Int4,
        chain_id -> Nullable<Int4>,
        address -> Varchar,
        name -> Varchar,
        symbol -> Varchar,
        decimals -> Int4,
        logo_uri -> Nullable<Varchar>,
        tags -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    signatures,
    tokens,
);
