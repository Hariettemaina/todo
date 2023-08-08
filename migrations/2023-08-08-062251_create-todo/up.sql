-- Your SQL goes here

CREATE TABLE IF NOT EXISTS todos (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL DEFAULT FALSE,
  description VARCHAR,
  due_date DATE,
  completed_date DATE,
  FOREIGN KEY (username) REFERENCES users(username)
);

ALTER TABLE todos
ADD CONSTRAINT fk_todos_username
FOREIGN KEY (username) REFERENCES users(username);