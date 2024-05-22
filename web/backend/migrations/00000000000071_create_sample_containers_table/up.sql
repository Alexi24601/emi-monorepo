CREATE TABLE IF NOT EXISTS sample_containers(
    id INTEGER PRIMARY KEY,
    barcode TEXT NOT NULL UNIQUE,
    -- state_id INTEGER NOT NULL REFERENCES sample_container_states(id),
    -- project_id INTEGER NOT NULL REFERENCES projects(id),
    category_id INTEGER NOT NULL REFERENCES sample_container_categories(id),
    created_by INTEGER NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
