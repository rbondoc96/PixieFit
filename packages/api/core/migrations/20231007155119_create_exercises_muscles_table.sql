CREATE TABLE IF NOT EXISTS exercises_muscles (
    id SMALLSERIAL PRIMARY KEY NOT NULL,
    exercise_id SMALLINT REFERENCES exercises(id) ON DELETE CASCADE NOT NULL,
    muscle_id SMALLINT REFERENCES muscles(id) ON DELETE CASCADE NOT NULL,
    target VARCHAR(15) NOT NULL,
    UNIQUE (exercise_id, muscle_id, target)
);
