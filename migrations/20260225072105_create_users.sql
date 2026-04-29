-- Add migration script here

-- Users table
CREATE TABLE IF NOT EXISTS users  (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
