ALTER TABLE question_notes
    ADD COLUMN parent_id BIGINT REFERENCES question_notes(id) ON DELETE CASCADE;

CREATE INDEX idx_question_notes_parent
    ON question_notes (parent_id)
    WHERE parent_id IS NOT NULL;
