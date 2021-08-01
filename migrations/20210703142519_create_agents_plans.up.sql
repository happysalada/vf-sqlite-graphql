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
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS plan_agents
(
    id          INTEGER PRIMARY KEY NOT NULL,
    agent_id    VARCHAR(100) NOT NULL REFERENCES agents(unique_name),
    plan_id     VARCHAR(26) NOT NULL REFERENCES plans(id),
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_plan_agents_plan_id_agent_id ON plan_agents (plan_id, agent_id)