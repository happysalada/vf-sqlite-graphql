-- Add up migration script here
CREATE TABLE IF NOT EXISTS labels 
(
    id                 VARCHAR(26) PRIMARY KEY NOT NULL,
    name               TEXT NOT NULL, 
    unique_name        TEXT NOT NULL UNIQUE,
    color              TEXT,
    inserted_at        INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);
