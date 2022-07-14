table! {
    addresses (address_id) {
        address_id -> Int4,
        street -> Text,
        city -> Text,
        state -> Text,
        zip -> Text,
        country -> Text,
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
        lastname -> Text,
        nickname -> Text,
        company -> Text,
        url -> Text,
        notes -> Text,
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
