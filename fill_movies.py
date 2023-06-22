import psycopg2
import requests
import sys
import os
from dotenv import load_dotenv
from urllib.parse import urlparse

class Movie:
    def __init__(self, movie_id: str, title: str, year: int, genres: list):
        self.movie_id = movie_id
        self.title = title
        self.year = year
        self.genres = genres
    
    def __str__(self):
        return f"Movie: {self.movie_id}, {self.title}, {self.year}, {self.genres}"

DEFAULT_MOVIES_COUNT = 200

def get_movies_count_from_args():
    if len(sys.argv) == 1:
        print("Filling with default movies count")
        movies_count = DEFAULT_MOVIES_COUNT
    else:
        num = int(sys.argv[1])
        movies_count = num
    return movies_count

def get_movies_data(count, url = "https://raw.githubusercontent.com/sidooms/MovieTweetings/master/latest/movies.dat"):
    print(f"Getting {count} movies")
    r = requests.get(url)
    movies_str_list = r.content.decode("utf-8").split("\n")
    movies_list = list()
    for i in range(count):
        parts = movies_str_list[i].split("::")
        if len(parts) < 3:
            continue
        movie_id = parts[0]
        title_and_year = parts[1].split('(')
        title = title_and_year[0]
        year = int(title_and_year[1][:-1])
        genres = parts[2].split("|")
        movies_list.append(Movie(movie_id, title, year, genres))
    return movies_list

def get_connection():
    db_url = os.getenv("DATABASE_URL")
    parsed_url = urlparse(db_url)
    username = parsed_url.username
    password = parsed_url.password
    database = parsed_url.path[1:]
    hostname = parsed_url.hostname
    port = parsed_url.port
    connection = psycopg2.connect(
        database = database,
        user = username,
        password = password,
        host = hostname,
        port = port
    )
    connection.autocommit = True
    return connection

def truncate(cursor: psycopg2.extensions.cursor):
    query = '''
        TRUNCATE TABLE movies CASCADE;
    '''
    cursor.execute(query)
    print("Movies table successfully truncated")


def fill_movies(cursor: psycopg2.extensions.cursor, movies: list):
    query = '''
        INSERT INTO movies (movie_id, title, year) VALUES (%s, %s, %s); 
    '''
    for movie in movies:
        cursor.execute(query, (movie.movie_id, movie.title, movie.year))
    print("Successfully filled movies table")

def fill_genres(cursor: psycopg2.extensions.cursor, movies: list):
    query = '''
        SELECT * FROM genre                                
    '''
    cursor.execute(query)
    genres_fetch = cursor.fetchall()
    genres_dict = dict()
    for genre in genres_fetch:
        genres_dict[genre[1]] = genre[0]
    
    query = '''
        INSERT INTO movie_genre (movie_id, genre_id) VALUES (%s, %s);
    '''
    for movie in movies:
        for genre in movie.genres:
            if genre == '':
                continue
            genre_id = genres_dict[genre] 
            cursor.execute(query, (movie.movie_id, genre_id))
    print("Successfully filled movie genres")


def main():
    load_dotenv()

    movies_count = get_movies_count_from_args()
    movies = get_movies_data(movies_count) 
    
    try:
        connection = get_connection()
        cursor = connection.cursor()
        cursor.execute("SELECT version();")
        record = cursor.fetchone()
        print("You are connected to - ", record, "\n")

        truncate(cursor)
        fill_movies(cursor, movies) 
        fill_genres(cursor, movies)

        print("Movies have been successfully commited to database")

    except Exception as error:
        print("Error while connecting to PostgreSQL", error)
    finally:
        if (connection):
            cursor.close()
            connection.close()
            print("PostgreSQL connection is closed")

if __name__ == "__main__":
    main()
