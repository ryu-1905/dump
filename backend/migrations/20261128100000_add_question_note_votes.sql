CREATE TABLE question_note_votes (
    note_id BIGINT NOT NULL REFERENCES question_notes(id) ON DELETE CASCADE,
    client_id TEXT NOT NULL REFERENCES clients(id) ON UPDATE CASCADE ON DELETE CASCADE,
    vote SMALLINT NOT NULL CHECK (vote IN (-1, 1)),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (note_id, client_id)
);

CREATE INDEX idx_question_note_votes_client
    ON question_note_votes (client_id);
