-- 1. Exams
CREATE TABLE exams (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL,
    title TEXT NOT NULL,
    vendor TEXT NOT NULL,
    description TEXT NOT NULL,
    description_vi TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_exams_updated_at ON exams (updated_at DESC);

-- 2. Topics (scoped per exam)
CREATE TABLE topics (
    exam_id TEXT NOT NULL REFERENCES exams(id) ON DELETE CASCADE,
    id TEXT NOT NULL,
    name TEXT NOT NULL,
    name_vi TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (exam_id, id)
);

CREATE INDEX idx_topics_exam_sort ON topics (exam_id, sort_order);

-- 3. Questions
CREATE TABLE questions (
    id TEXT PRIMARY KEY,
    exam_id TEXT NOT NULL,
    topic_id TEXT NOT NULL,
    question_index INTEGER NOT NULL CHECK (question_index >= 1),
    prompt TEXT NOT NULL,
    prompt_vi TEXT,
    multiple_choice BOOLEAN NOT NULL DEFAULT FALSE,
    options JSONB NOT NULL CHECK (jsonb_typeof(options) = 'array'),
    correct_option_ids TEXT[] NOT NULL CHECK (cardinality(correct_option_ids) >= 1),
    explanation TEXT NOT NULL,
    explanation_vi TEXT,
    keywords TEXT[] NOT NULL DEFAULT '{}',
    keywords_vi TEXT[] NOT NULL DEFAULT '{}',
    images JSONB NOT NULL DEFAULT '[]' CHECK (jsonb_typeof(images) = 'array'),
    FOREIGN KEY (exam_id, topic_id) REFERENCES topics(exam_id, id) ON DELETE CASCADE,
    UNIQUE (exam_id, id),
    UNIQUE (exam_id, question_index)
);

CREATE INDEX idx_questions_exam_topic_index
    ON questions (exam_id, topic_id, question_index);

-- 4. Client progress per exam (X-Client-Id)
CREATE TABLE client_exam_progress (
    client_id UUID NOT NULL,
    exam_id TEXT NOT NULL REFERENCES exams(id) ON DELETE CASCADE,
    last_question_index INTEGER CHECK (last_question_index IS NULL OR last_question_index >= 1),
    best_score_percent INTEGER CHECK (best_score_percent IS NULL OR (best_score_percent >= 0 AND best_score_percent <= 100)),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (client_id, exam_id)
);

CREATE INDEX idx_client_exam_progress_exam
    ON client_exam_progress (exam_id);

-- 5. Client submitted answers per question
CREATE TABLE client_answers (
    client_id UUID NOT NULL,
    question_id TEXT NOT NULL,
    exam_id TEXT NOT NULL,
    question_index INTEGER NOT NULL CHECK (question_index >= 1),
    selected_option_ids TEXT[] NOT NULL,
    is_correct BOOLEAN NOT NULL,
    answered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (client_id, question_id),
    FOREIGN KEY (exam_id, question_id) REFERENCES questions(exam_id, id) ON DELETE CASCADE
);

CREATE INDEX idx_client_answers_client_exam
    ON client_answers (client_id, exam_id, question_index);
