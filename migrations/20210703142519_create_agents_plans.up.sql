-- Add up migration script here
CREATE TABLE IF NOT EXISTS agents
(
    id          VARCHAR(26) PRIMARY KEY NOT NULL,
    unique_name TEXT NOT NULL UNIQUE,
    name       TEXT NOT NULL, 
    email      TEXT,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS plans
(
    id          VARCHAR(26) PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL, 
    description TEXT,
    agent_id    VARCHAR(26) NOT NULL REFERENCES agents(id),
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);