-- Your SQL goes here
CREATE TABLE IF NOT EXISTS spectra (
    id INTEGER PRIMARY KEY,
    notes TEXT,
    spectra_collection_id INTEGER NOT NULL REFERENCES spectra_collections(id) ON DELETE CASCADE,
    created_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
