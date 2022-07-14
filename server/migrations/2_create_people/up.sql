-- CREATE PEOPLE
CREATE TABLE people (
  person_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(person_id),

  firstname TEXT NOT NULl,
  lastname TEXT NOT NULl,

  -- change these back to nullable
  nickname TEXT NOT NULl,
  company TEXT NOT NULl,
  url TEXT NOT NULl,
  notes TEXT NOT NULl,
  --
  favorite BOOLEAN NOT NULL,
  active BOOLEAN NOT NULL,

  --
  address_id INT NOT NULL,
  --

  CONSTRAINT fk_address FOREIGN KEY(address_id) REFERENCES addresses(address_id) ON DELETE CASCADE
);
INSERT INTO 
  people(firstname,lastname,nickname,company,url,notes,favorite,active,address_id) 
  VALUES('skyler','favors','sky','MSCA','skylerfavors.com','God tier league player',true,true,1);
