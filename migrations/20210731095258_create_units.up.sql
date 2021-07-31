-- Add up migration script here
CREATE TABLE IF NOT EXISTS units
(
    id           VARCHAR(26) PRIMARY KEY NOT NULL,
    label        TEXT NOT NULL, 
    inserted_at  INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);