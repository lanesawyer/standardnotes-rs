table! {
    items (uuid) {
        uuid -> Text,
        content -> Text,
        content_type -> Text,
        enc_item_key -> Text,
        deleted -> Bool,
        created_at -> Text,
        updated_at -> Text,
    }
}

table! {
    users (email) {
        email -> Text,
        password -> Text,
        pw_cost -> Text,
        pw_nonce -> Text,
        version -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    users,
);
