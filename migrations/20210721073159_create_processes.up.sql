-- Add up migration script here
CREATE TABLE IF NOT EXISTS processes
(
    id          VARCHAR(26) PRIMARY KEY NOT NULL,
    title       TEXT NOT NULL, 
    description TEXT,
    plan_id     VARCHAR(26) REFERENCES plans(id),
    start_at    INT,
    due_at      INT,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);


CREATE TABLE IF NOT EXISTS process_labels
(
    id          INTEGER PRIMARY KEY NOT NULL,
    process_id  VARCHAR(26) REFERENCES processes(id) NOT NULL,
    label_id    VARCHAR(26) REFERENCES labels(id) NOT NULL,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_process_labels_process_id_label_id ON process_labels (process_id, label_id);

CREATE TABLE IF NOT EXISTS process_agents
(
    id          INTEGER PRIMARY KEY NOT NULL,
    process_id  VARCHAR(26) REFERENCES processes(id) NOT NULL,
    agent_id    VARCHAR(26) REFERENCES agents(id) NOT NULL,
    inserted_at INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_process_agents_process_id_agent_id ON process_agents (process_id, agent_id)