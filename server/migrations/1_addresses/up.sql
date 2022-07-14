-- CREATE ADDRESSES
CREATE TABLE addresses (
  address_id INT GENERATED ALWAYS AS IDENTITY,
  PRIMARY KEY(address_id),

  street TEXT NOT NULl,
  city TEXT NOT NULl,
  state TEXT NOT NULl,
  zip TEXT NOT NULl, country TEXT NOT NULl
);
INSERT INTO addresses(street,city,state,zip,country) 
  VALUES('308 Woodward Drive','Ocean Springs','MS','39564','US');
