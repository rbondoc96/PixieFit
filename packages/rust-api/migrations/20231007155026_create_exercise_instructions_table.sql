CREATE TABLE IF NOT EXISTS exercise_instructions (
    exercise_id BIGINT REFERENCES exercises(id) ON DELETE CASCADE NOT NULL,
    "order" SMALLINT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,

    PRIMARY KEY (exercise_id, "order")
);
