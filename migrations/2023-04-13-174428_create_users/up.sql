--- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  unique_id VARCHAR NOT NULL,
  UNIQUE (email),
  UNIQUE (name)
);

-- create a master user
INSERT INTO users (name, email, password, unique_id)
VALUES ('placeholder', 'placeholder email',
'placeholder password', 'placeholder unique id');

