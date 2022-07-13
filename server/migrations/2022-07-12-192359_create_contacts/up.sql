-- CREATE ADDRESSES
CREATE TABLE addresses (
  address_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(address_id),

  street TEXT NOT NULl,
  city TEXT NOT NULl,
  state TEXT NOT NULl,
  zip TEXT NOT NULl, country TEXT NOT NULl
);

-- CREATE EMAILS
CREATE TABLE emails (
  email_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(email_id),

  email TEXT NOT NULl
);

-- CREATE Phone Numbers
CREATE TABLE phone_numbers (
  phone_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(phone_id),

  num TEXT NOT NULL
);

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

-- LINK a person to multiple emails
CREATE TABLE emails_link (
  emails_link_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(emails_link_id),

  person_id INT NOT NULL,
  email_id INT NOT NULL,
  
  CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES people(person_id) ON DELETE CASCADE,
  CONSTRAINT fk_email FOREIGN KEY(email_id) REFERENCES emails(email_id) ON DELETE CASCADE
);

-- LINK a person to multiple phone numbers
CREATE TABLE phone_link (
  phone_link_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(phone_link_id),

  person_id INT NOT NULL,
  phone_id INT NOT NULL,
  
  CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES people(person_id) ON DELETE CASCADE,
  CONSTRAINT fk_phone FOREIGN KEY(phone_id) REFERENCES phone_numbers(phone_id) ON DELETE CASCADE 
);

-- DEFAULT TEST DATA
INSERT INTO addresses(street,city,state,zip,country) 
  VALUES('308 Woodward Drive','Ocean Springs','MS','39564','US');

INSERT INTO emails(email) VALUES('skylerafavors@gmail.com');
INSERT INTO emails(email) VALUES('sky86one@gmail.com');

INSERT INTO phone_numbers(num) VALUES('228-282-4795');
INSERT INTO phone_numbers(num) VALUES('228-867-5309');

INSERT INTO 
  people(firstname,lastname,nickname,company,url,notes,favorite,active,address_id) 
  VALUES('skyler','favors','sky','MSCA','skylerfavors.com','God tier league player',true,true,1);

INSERT INTO emails_link(person_id,email_id) VALUES(1,1);
INSERT INTO emails_link(person_id,email_id) VALUES(1,2);
INSERT INTO phone_link(person_id,phone_id) VALUES(1,1);
INSERT INTO phone_link(person_id,phone_id) VALUES(1,2);
