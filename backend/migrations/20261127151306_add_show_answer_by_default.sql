ALTER TABLE questions
    ADD COLUMN show_answer_by_default BOOLEAN NOT NULL DEFAULT FALSE;

ALTER TABLE questions
    DROP CONSTRAINT questions_correct_option_ids_check,
    ADD CONSTRAINT questions_correct_option_ids_check
        CHECK (show_answer_by_default OR cardinality(correct_option_ids) >= 1);
