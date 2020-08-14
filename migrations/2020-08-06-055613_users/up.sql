CREATE TABLE users (
    email TEXT PRIMARY KEY,
    password TEXT NOT NULL,
    pw_cost BIGINT NOT NULL,
    pw_nonce TEXT NOT NULL,
    version TEXT NOT NULL
)
