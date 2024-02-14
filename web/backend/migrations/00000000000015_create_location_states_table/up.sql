-- SQL defining the location states table.
-- A location state is a state that a location may be in.
-- A location state is used to describe the state of a location, such as destroyed, at risk, or safe.
CREATE TABLE location_states (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    font_awesome_icon VARCHAR(255),
    editable_id INTEGER NOT NULL REFERENCES editables(id) ON DELETE CASCADE
);