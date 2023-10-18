CREATE TABLE IF NOT EXISTS exercises_muscles (
    exercise_id BIGINT REFERENCES exercises(id) ON DELETE CASCADE NOT NULL,
    muscle_id BIGINT REFERENCES muscles(id) ON DELETE CASCADE NOT NULL,
    target VARCHAR(15) NOT NULL,
    PRIMARY KEY (exercise_id, muscle_id)
);
