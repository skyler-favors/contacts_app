table! {
    contacts (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Text,
        phone_number -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        age -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    contacts,
    users,
);
