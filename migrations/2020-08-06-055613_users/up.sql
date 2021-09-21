CREATE TABLE users (
    uuid TEXT PRIMARY KEY, -- TODO: Uuid
    api TEXT NOT NULL,
    created TEXT NOT NULL,
    email TEXT NOT NULL,
    identifier TEXT NOT NULL,
    origination TEXT NOT NULL,
    password TEXT NOT NULL,
    pw_nonce TEXT NOT NULL,
    version TEXT NOT NULL
)
