CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  published_at TEXT NOT NULL,
  quantity INT NOT NULL,
  item_id INTEGER NOT NULL REFERENCES items(id),
  table_id INTEGER NOT NULL REFERENCES tables(id)
);
