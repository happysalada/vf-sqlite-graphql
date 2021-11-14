-- Add up migration script here
CREATE TABLE IF NOT EXISTS agent_relation_types
(
    id            VARCHAR(26) PRIMARY KEY NOT NULL,
    name          VARCHAR(100) NOT NULL,
    inserted_at   INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_name ON agent_relation_types (name);

CREATE TABLE IF NOT EXISTS agent_relations
(
    id                       VARCHAR(26) PRIMARY KEY NOT NULL,
    subject_id               VARCHAR(26) NOT NULL REFERENCES agents(id),
    object_id                VARCHAR(26) NOT NULL REFERENCES agents(id),
    agent_relation_type_id   VARCHAR(26) NOT NULL REFERENCES agent_relation_types(id),
    start_at                 INT DEFAULT CURRENT_TIMESTAMP NOT NULL,
    end_at                   INT,
    inserted_at              INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS unique_subject_object_agent_relations ON agent_relations (subject_id, object_id)
