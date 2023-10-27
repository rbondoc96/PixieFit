CREATE TABLE IF NOT EXISTS exercise_instructions (
    id SMALLSERIAL PRIMARY KEY NOT NULL,
    exercise_id SMALLINT REFERENCES exercises(id) ON DELETE CASCADE NOT NULL,
    sequence_number SMALLINT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,

    UNIQUE (exercise_id, sequence_number)
);
