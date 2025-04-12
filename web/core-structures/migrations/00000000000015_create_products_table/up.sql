CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
	name TEXT NOT NULL UNIQUE,
	grams REAL NOT NULL CHECK (must_be_strictly_positive_f32(grams)),
	deprecation_date TIMESTAMP WITH TIME ZONE,
	brand_id INTEGER NOT NULL REFERENCES brands(id),
	created_by INTEGER NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INTEGER NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);