-- Add migration script here
DROP TABLE habit_days;

CREATE TABLE IF NOT EXISTS habit_days (
    habit_id INTEGER NOT NULL,
    day INTEGER NOT NULL,
    PRIMARY KEY (habit_id, day)

    FOREIGN KEY (habit_id) REFERENCES habits(id) ON DELETE CASCADE
);
