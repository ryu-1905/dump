CREATE TABLE question_notes (
    id BIGSERIAL PRIMARY KEY,
    exam_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    client_id TEXT NOT NULL REFERENCES clients(id) ON UPDATE CASCADE ON DELETE CASCADE,
    content TEXT NOT NULL CHECK (char_length(btrim(content)) BETWEEN 1 AND 2000),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (exam_id, question_id) REFERENCES questions(exam_id, id) ON DELETE CASCADE
);

CREATE INDEX idx_question_notes_question_created
    ON question_notes (exam_id, question_id, created_at DESC, id DESC);

CREATE INDEX idx_question_notes_client
    ON question_notes (client_id);
