-- Update users table to support 'view' role
-- SQLite doesn't support ALTER CHECK, so we recreate the table

CREATE TABLE IF NOT EXISTS users_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    updated_ts BIGINT NOT NULL DEFAULT (unixepoch()),
    email TEXT NOT NULL UNIQUE,
    nickname TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('admin', 'user', 'view')) DEFAULT 'view'
);

INSERT INTO users_new SELECT id, created_ts, updated_ts, email, nickname, password_hash, role FROM users;
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
