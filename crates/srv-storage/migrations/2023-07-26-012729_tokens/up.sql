-- Your SQL goes here
CREATE TABLE tokens(
	id SERIAL PRIMARY KEY,
	chain_id INTEGER,
	address VARCHAR NOT NULL,
	name VARCHAR NOT NULL,
	symbol VARCHAR NOT NULL,
	decimals INTEGER NOT NULL,
	logo_uri VARCHAR,
	tags TEXT[],
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX token_address ON tokens (chain_id, address);