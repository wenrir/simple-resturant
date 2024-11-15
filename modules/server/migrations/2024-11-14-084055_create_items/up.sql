CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  description TEXT NOT NULL,
  quantity INT NOT NULL,
  order_id INTEGER NOT NULL REFERENCES orders(id)
);
