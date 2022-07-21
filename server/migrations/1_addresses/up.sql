-- CREATE ADDRESSES
CREATE TABLE addresses (
  address_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(address_id),

  street TEXT,
  city TEXT,
  state TEXT,
  zip TEXT,
  country TEXT
);
INSERT INTO addresses(street,city,state,zip,country) 
  VALUES('308 Woodward Drive','Ocean Springs','MS','39564','US');
