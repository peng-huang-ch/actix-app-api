-- Your SQL goes here
CREATE TABLE signatures
(
  id SERIAL PRIMARY KEY,
  signature VARCHAR NOT NULL,
  bytes VARCHAR NOT NULL,
  abi TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX signatures_signature ON signatures (signature);