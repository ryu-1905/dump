-- v2 Microsoft-exam-style schema:
-- - Native ENUM question_type
-- - Normalized case_studies table
-- - Unified polymorphic JSONB (spec, answer_key, submitted_answer)
-- - Independent client_question_flags table (Mark for review before or after answering)
-- - Covering index (INCLUDE) and partial indexes for fast navigator & review screens

CREATE TYPE question_type AS ENUM (
    'single_choice',
    'multiple_choice',
    'hotspot',
    'drag_drop',
    'simulation'
);

CREATE TABLE case_studies (
    exam_id TEXT NOT NULL REFERENCES exams(id) ON DELETE CASCADE,
    id TEXT NOT NULL,
    title TEXT NOT NULL,
    title_vi TEXT,
    sections JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(sections) = 'array'),
    images JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(images) = 'array'),
    PRIMARY KEY (exam_id, id)
);

-- Recreate client_answers and questions cleanly (user explicitly approved dropping old v1 question/answer data)
DROP TABLE IF EXISTS client_answers;
DROP TABLE IF EXISTS questions;

CREATE TABLE questions (
    id TEXT PRIMARY KEY,
    exam_id TEXT NOT NULL,
    topic_id TEXT NOT NULL,
    case_study_id TEXT,
    question_index INTEGER NOT NULL CHECK (question_index >= 1),
    question_type question_type NOT NULL,
    prompt TEXT NOT NULL,
    prompt_vi TEXT,
    spec JSONB NOT NULL DEFAULT '{}'::jsonb CHECK (jsonb_typeof(spec) = 'object'),
    answer_key JSONB,
    max_points SMALLINT NOT NULL DEFAULT 1 CHECK (max_points >= 0),
    explanation TEXT NOT NULL,
    explanation_vi TEXT,
    keywords TEXT[] NOT NULL DEFAULT '{}',
    keywords_vi TEXT[] NOT NULL DEFAULT '{}',
    images JSONB NOT NULL DEFAULT '[]'::jsonb CHECK (jsonb_typeof(images) = 'array'),
    FOREIGN KEY (exam_id, topic_id) REFERENCES topics(exam_id, id) ON DELETE CASCADE,
    FOREIGN KEY (exam_id, case_study_id) REFERENCES case_studies(exam_id, id) ON DELETE SET NULL,
    UNIQUE (exam_id, id),
    UNIQUE (exam_id, question_index),
    CHECK (
        (question_type = 'simulation' AND answer_key IS NULL AND max_points = 0)
        OR (question_type = 'single_choice' AND jsonb_typeof(answer_key) = 'array' AND jsonb_array_length(answer_key) = 1 AND max_points = 1)
        OR (question_type = 'multiple_choice' AND jsonb_typeof(answer_key) = 'array' AND jsonb_array_length(answer_key) >= 2 AND max_points >= 1)
        OR (question_type IN ('hotspot', 'drag_drop') AND jsonb_typeof(answer_key) = 'object' AND max_points >= 1)
    )
);

CREATE INDEX idx_questions_exam_topic_index
    ON questions (exam_id, topic_id, question_index);

CREATE INDEX idx_questions_case_study
    ON questions (exam_id, case_study_id)
    WHERE case_study_id IS NOT NULL;

CREATE TABLE client_answers (
    client_id TEXT NOT NULL REFERENCES clients(id) ON UPDATE CASCADE ON DELETE CASCADE,
    question_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    question_index INTEGER NOT NULL CHECK (question_index >= 1),
    submitted_answer JSONB NOT NULL,
    earned_points SMALLINT NOT NULL DEFAULT 0 CHECK (earned_points >= 0),
    max_points SMALLINT NOT NULL DEFAULT 1 CHECK (max_points >= 0 AND earned_points <= max_points),
    is_correct BOOLEAN NOT NULL,
    answered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (client_id, question_id),
    FOREIGN KEY (exam_id, question_id) REFERENCES questions(exam_id, id) ON DELETE CASCADE
);

CREATE INDEX idx_client_answers_nav
    ON client_answers (client_id, exam_id, question_index)
    INCLUDE (is_correct, earned_points, max_points);

CREATE INDEX idx_client_answers_wrong
    ON client_answers (client_id, exam_id, question_index)
    WHERE is_correct = FALSE;

CREATE TABLE client_question_flags (
    client_id TEXT NOT NULL REFERENCES clients(id) ON UPDATE CASCADE ON DELETE CASCADE,
    question_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    question_index INTEGER NOT NULL CHECK (question_index >= 1),
    flagged_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (client_id, question_id),
    FOREIGN KEY (exam_id, question_id) REFERENCES questions(exam_id, id) ON DELETE CASCADE
);

CREATE INDEX idx_client_flags_nav
    ON client_question_flags (client_id, exam_id, question_index);
