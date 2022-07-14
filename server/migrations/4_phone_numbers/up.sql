-- CREATE Phone Numbers
CREATE TABLE phone_numbers (
  phone_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(phone_id),

  person_id INT NOT NULL,
  num TEXT NOT NULL,

  CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES people(person_id) ON DELETE CASCADE
);
INSERT INTO phone_numbers(person_id,num) VALUES(1,'228-282-4795');
INSERT INTO phone_numbers(person_id,num) VALUES(1,'228-867-5309');
