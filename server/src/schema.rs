table! {
    addresses (address_id) {
        address_id -> Int4,
        street -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        zip -> Nullable<Text>,
        country -> Nullable<Text>,
    }
}

table! {
    emails (email_id) {
        email_id -> Int4,
        person_id -> Int4,
        email -> Text,
    }
}

table! {
    people (person_id) {
        person_id -> Int4,
        firstname -> Text,
        lastname -> Nullable<Text>,
        nickname -> Nullable<Text>,
        company -> Nullable<Text>,
        url -> Nullable<Text>,
        notes -> Nullable<Text>,
        favorite -> Bool,
        active -> Bool,
        address_id -> Int4,
    }
}

table! {
    phone_numbers (phone_id) {
        phone_id -> Int4,
        person_id -> Int4,
        num -> Text,
    }
}

joinable!(people -> addresses (address_id));

allow_tables_to_appear_in_same_query!(
    addresses,
    emails,
    people,
    phone_numbers,
);
