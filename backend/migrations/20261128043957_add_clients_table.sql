CREATE TABLE clients (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO clients (id)
SELECT DISTINCT client_id::text FROM client_exam_progress
UNION
SELECT DISTINCT client_id::text FROM client_answers
ON CONFLICT (id) DO NOTHING;

ALTER TABLE client_exam_progress
    ALTER COLUMN client_id TYPE TEXT USING client_id::text,
    ADD CONSTRAINT client_exam_progress_client_id_fkey
        FOREIGN KEY (client_id) REFERENCES clients(id)
        ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE client_answers
    ALTER COLUMN client_id TYPE TEXT USING client_id::text,
    ADD CONSTRAINT client_answers_client_id_fkey
        FOREIGN KEY (client_id) REFERENCES clients(id)
        ON UPDATE CASCADE ON DELETE CASCADE;
