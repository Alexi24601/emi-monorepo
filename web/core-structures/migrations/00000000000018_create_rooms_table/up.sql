CREATE TABLE IF NOT EXISTS rooms (
	id SERIAL PRIMARY KEY,
	name TEXT NOT NULL,
	description TEXT NOT NULL,
	qrcode UUID NOT NULL UNIQUE,
	addresses_id INT NOT NULL REFERENCES addresses(id),
	geolocation GEOGRAPHY(POINT, 4326) NOT NULL,
	created_by INT NOT NULL REFERENCES users(id),
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_by INT NOT NULL REFERENCES users(id),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);