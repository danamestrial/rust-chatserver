CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  username VARCHAR UNIQUE NOT NULL,
  password TEXT NOT NULL,
  rooms Text NOT NULL
)