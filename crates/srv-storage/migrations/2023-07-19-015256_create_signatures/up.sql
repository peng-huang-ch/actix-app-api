-- Your SQL goes here
CREATE TABLE signatures(
  id SERIAL PRIMARY KEY,
  text VARCHAR NOT NULL,
  hash VARCHAR NOT NULL,
  abi TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX signatures_hash_text ON signatures (hash, text);