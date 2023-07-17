-- TODO: add source and target references
CREATE TABLE seeded_recs (
	id SERIAL PRIMARY KEY NOT NULL,
	created TIMESTAMP NOT NULL,
	source VARCHAR(16) NOT NULL,
	target VARCHAR(16) NOT NULL,
	support DECIMAL NOT NULL,
	confidence DECIMAL NOT NULL
);
