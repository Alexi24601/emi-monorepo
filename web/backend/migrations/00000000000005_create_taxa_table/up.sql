-- Your SQL goes here
CREATE TABLE taxa (
    id INTEGER PRIMARY KEY REFERENCES editables(id) ON DELETE CASCADE,
    name VARCHAR(80) NOT NULL,
    description VARCHAR(255) NOT NULL
);