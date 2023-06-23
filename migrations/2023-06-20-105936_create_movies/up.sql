CREATE TABLE movies (
	movie_id VARCHAR(16) NOT NULL PRIMARY KEY,
	title VARCHAR(512) NOT NULL,
	year INTEGER NULL
);

CREATE TABLE genre (
	id SERIAL NOT NULL PRIMARY KEY,
	name VARCHAR(64) NOT NULL
);

INSERT INTO genre (name) VALUES ('Documentary');
INSERT INTO genre (name) VALUES ('Short');
INSERT INTO genre (name) VALUES ('Horror');
INSERT INTO genre (name) VALUES ('Comedy');
INSERT INTO genre (name) VALUES ('Action');
INSERT INTO genre (name) VALUES ('Adventure');
INSERT INTO genre (name) VALUES ('Fantasy');
INSERT INTO genre (name) VALUES ('Sci-Fi');
INSERT INTO genre (name) VALUES ('Crime');
INSERT INTO genre (name) VALUES ('Western');
INSERT INTO genre (name) VALUES ('Drama');
INSERT INTO genre (name) VALUES ('Romance');
INSERT INTO genre (name) VALUES ('History');
INSERT INTO genre (name) VALUES ('Family');
INSERT INTO genre (name) VALUES ('War');
INSERT INTO genre (name) VALUES ('Sport');
INSERT INTO genre (name) VALUES ('Biography');
INSERT INTO genre (name) VALUES ('Mystery');
INSERT INTO genre (name) VALUES ('Thriller');
INSERT INTO genre (name) VALUES ('Animation');
INSERT INTO genre (name) VALUES ('Music');
INSERT INTO genre (name) VALUES ('Musical');
INSERT INTO genre (name) VALUES ('Film-Noir');
INSERT INTO genre (name) VALUES ('Adult');
INSERT INTO genre (name) VALUES ('Talk-Show');
INSERT INTO genre (name) VALUES ('Documentary');
INSERT INTO genre (name) VALUES ('News');
INSERT INTO genre (name) VALUES ('Reality-TV');
INSERT INTO genre (name) VALUES ('Game-Show');

CREATE TABLE movie_genre (
	id SERIAL NOT NULL PRIMARY KEY,
	movie_id VARCHAR(16) NOT NULL REFERENCES movies(movie_id),
	genre_id SERIAL NOT NULL REFERENCES genre(id)
);
