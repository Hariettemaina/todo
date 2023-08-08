-- Your SQL goes here

CREATE TABLE IF NOT EXISTS category_lookup (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR
);