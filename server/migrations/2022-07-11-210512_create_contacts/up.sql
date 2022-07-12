CREATE TABLE contacts (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  phone_number TEXT NOT NULL
);
INSERT INTO contacts(user_id,name,phone_number) VALUES(1,'Bobby Jim','2288675309');
