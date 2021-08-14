-- Add up migration script here
CREATE TABLE IF NOT EXISTS resource_specifications
(
    id                 VARCHAR(26) PRIMARY KEY NOT NULL,
    name               TEXT NOT NULL, 
    unique_name        TEXT NOT NULL,
    agent_unique_name  VARCHAR(100) NOT NULL REFERENCES agents(unique_name),
    inserted_at        INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_resource_specifications_unique_name_agent_unique_name ON resource_specifications (unique_name, agent_unique_name)
