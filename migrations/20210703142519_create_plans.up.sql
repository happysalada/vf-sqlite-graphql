-- Add up migration script here
CREATE TABLE IF NOT EXISTS plans
(
    id          VARCHAR(26) PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL, 
    description TEXT,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);