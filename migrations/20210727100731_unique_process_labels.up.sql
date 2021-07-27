-- Add up migration script here
CREATE UNIQUE INDEX IF NOT EXISTS unique_process_labels_process_id_label_id ON process_labels (process_id, label_id)
