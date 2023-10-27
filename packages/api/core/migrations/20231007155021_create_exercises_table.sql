CREATE TABLE IF NOT EXISTS exercises (
    id SMALLSERIAL PRIMARY KEY NOT NULL,
    external_id SMALLINT UNIQUE,
    ulid VARCHAR UNIQUE DEFAULT generate_ulid() NOT NULL,
    type VARCHAR NOT NULL,
    target_muscle_group_id SMALLINT REFERENCES muscle_groups(id) ON DELETE CASCADE,
    equipment_id SMALLINT REFERENCES exercise_equipment(id) ON DELETE CASCADE,
    name VARCHAR UNIQUE NOT NULL,
    name_alternative VARCHAR,
    description TEXT,
    mechanic VARCHAR,
    force VARCHAR,
    measurement VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL
);
