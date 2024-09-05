use diesel::prelude::*;
use diesel::table;
use diesel::allow_tables_to_appear_in_same_query;

table! {
    users (id) {
        id -> Uuid,
        full_name -> Varchar,
        dob -> Date,
        profile_picture -> Nullable<Varchar>,
        active -> Bool,
        password_hash -> Varchar,
        google_id -> Nullable<Varchar>,
        role -> Varchar,
    }
}

table! {
    properties (id) {
        id -> Uuid,
        name -> Varchar,
        location -> Varchar,
        active -> Bool,
        price -> Numeric,
        type -> Varchar,
        user_id -> Uuid,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    properties,
);
