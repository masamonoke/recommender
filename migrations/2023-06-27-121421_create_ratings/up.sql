CREATE TABLE ratings (
	id SERIAL NOT NULL PRIMARY KEY,
	user_id VARCHAR(16) NOT NULL,
	movie_id VARCHAR(16) NOT NULL,
	rating DECIMAL NOT NULL,
	rating_timestamp TIMESTAMP NOT NULL,
	rating_type VARCHAR(8) NOT NULL
);
--test
INSERT INTO ratings (user_id, movie_id, rating, rating_timestamp, rating_type) VALUES(
	'1',
	'0114508',
	8.1,
	'2013-10-05 21:00:50',
	'explicit'
);
