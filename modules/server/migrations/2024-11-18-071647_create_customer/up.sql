CREATE TABLE customers(
  id SERIAL PRIMARY KEY,
  checked_in_time TEXT NOT NULL,
  table_number INT NOT NULL,
  total INTEGER NOT NULL
);
