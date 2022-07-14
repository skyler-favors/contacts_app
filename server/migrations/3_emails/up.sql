-- CREATE EMAILS
CREATE TABLE emails (
  email_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(email_id),

  person_id INT NOT NULL,
  email TEXT NOT NULl,

  CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES people(person_id) ON DELETE CASCADE
);
INSERT INTO emails(person_id,email) VALUES(1,'skylerafavors@gmail.com');
INSERT INTO emails(person_id,email) VALUES(1,'sky86one@gmail.com');
