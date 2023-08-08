-- Your SQL goes here
-- Your SQL goes here

CREATE TABLE IF NOT EXISTS category (
  id SERIAL PRIMARY KEY,
  todo_id INT NOT NULL,
  category_id INT NOT NULL,
  FOREIGN KEY (todo_id) REFERENCES todos(id),
  FOREIGN KEY (category_id) REFERENCES category_lookup(id)
);

ALTER TABLE category
ADD CONSTRAINT fk_category_todo_id
FOREIGN KEY (todo_id) REFERENCES todos(id);

ALTER TABLE category
ADD CONSTRAINT fk_category_category_id
FOREIGN KEY (category_id) REFERENCES category_lookup(id);