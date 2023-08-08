-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
  username VARCHAR PRIMARY KEY,
  display_name VARCHAR,
  password VARCHAR NOT NULL,
  email_address VARCHAR UNIQUE NOT NULL,
  email_verified BOOLEAN NOT NULL DEFAULT FALSE,
  email_verification_code UUID NOT NULL,
  email_verification_code_expiry TIMESTAMP NOT NULL
);