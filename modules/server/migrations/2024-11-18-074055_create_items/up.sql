CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  description TEXT NOT NULL,
  estimated_minutes INT NOT NULL,
  price INT NOT NULL
);
