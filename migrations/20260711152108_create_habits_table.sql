-- Add migration script here
CREATE TABLE IF NOT EXISTS habits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    priority INTEGER,
    repeat_days INTEGER NOT NULL,
    last_edited DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_username ON habits (username);
CREATE INDEX idx_repeat_days ON habits (repeat_days);
