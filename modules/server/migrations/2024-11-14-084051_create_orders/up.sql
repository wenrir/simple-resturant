CREATE TABLE orders (
  id SERIAL PRIMARY KEY,
  table_number INT NOT NULL,
  published_at TIMESTAMP NOT NULL
);
