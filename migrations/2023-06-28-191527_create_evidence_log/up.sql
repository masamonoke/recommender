CREATE TABLE evidence_log (
	id SERIAL PRIMARY KEY NOT NULL,
	created TIMESTAMP NOT NULL,
	content_id VARCHAR(16) NOT NULL REFERENCES movies(movie_id),
	event VARCHAR(200) NOT NULL,
	session_id VARCHAR(128) NOT NULL,
	user_id INTEGER NOT NULL REFERENCES users(id)
);
-- example
--INSERT INTO evidence_log (created, content_id, event, session_id, user_id) 
--	VALUES ('2023-06-25 08:31:36.680085', '10155932', 'details', 'ad4ed21a-1332-11ee-997c-0242ac140002', '46678683991');
