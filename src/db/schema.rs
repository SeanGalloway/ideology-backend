table! {
    account (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        phone -> Nullable<Varchar>,
        password -> Varchar,
    }
}

table! {
    homepage_text (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
    }
}

table! {
    user_session (session_key) {
        session_key -> Uuid,
        user_id -> Int4,
        is_valid -> Bool,
        creation_timestamp -> Timestamp,
    }
}

joinable!(user_session -> account (user_id));

allow_tables_to_appear_in_same_query!(
    account,
    homepage_text,
    user_session,
);
