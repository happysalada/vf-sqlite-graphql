-- Add up migration script here
CREATE TABLE IF NOT EXISTS actions
(
    id           VARCHAR(26) PRIMARY KEY NOT NULL,
    name         TEXT NOT NULL UNIQUE, 
    description  TEXT,
    input_output VARCHAR(6),
    inserted_at  INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);