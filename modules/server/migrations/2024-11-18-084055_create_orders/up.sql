CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  table_number INT NOT NULL,
  published_at TEXT NOT NULL,
  quantity INT NOT NULL,
  item_id INTEGER NOT NULL REFERENCES items(id),
  customer_id INTEGER NOT NULL REFERENCES customers(id)
);
