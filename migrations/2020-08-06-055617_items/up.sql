CREATE TABLE items (
    uuid TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    content_type TEXT NOT NULL,
    enc_item_key TEXT NOT NULL,
    deleted BOOLEAN NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
)
