-- Add up migration script here
CREATE TABLE IF NOT EXISTS commitments 
(
    id                        VARCHAR(26) PRIMARY KEY NOT NULL,
    description               TEXT,
    process_id                VARCHAR(26) REFERENCES processes(id),
    action_id                 VARCHAR(26) REFERENCES actions(id),
    assigned_agent_id         VARCHAR(26) REFERENCES agents(id),
    quantity                  INT,
    unit_id                   VARCHAR(26) REFERENCES units(id),
    resource_specification_id VARCHAR(26) REFERENCES resource_specifications(id),
    due_at                    INT,
    inserted_at               INT DEFAULT CURRENT_TIMESTAMP NOT NULL
);

