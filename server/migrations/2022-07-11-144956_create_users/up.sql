CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULl,
  email TEXT NOT NULL,
  age INTEGER NOT NULL
);
INSERT INTO users(name,email,age) VALUES('skyler favors','skylerafavors@gmail.com',21);
