CREATE TABLE IF NOT EXISTS exercises (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    ulid VARCHAR UNIQUE DEFAULT generate_ulid() NOT NULL,
    type VARCHAR NOT NULL,
    target_muscle_group_id INTEGER REFERENCES muscle_groups(id) ON DELETE CASCADE NOT NULL,
    name VARCHAR UNIQUE NOT NULL,
    name_alternative VARCHAR,
    description TEXT,
    equipment VARCHAR REFERENCES exercise_equipment(name) ON DELETE CASCADE NOT NULL,
    mechanic VARCHAR NOT NULL,
    force VARCHAR NOT NULL,
    measurement VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);
