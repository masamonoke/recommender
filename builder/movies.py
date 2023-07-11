import requests
import psycopg2

from dotenv import load_dotenv
from urllib.parse import urlparse
from utils import get_connection, truncate
from tqdm import tqdm

class Movie:
    def __init__(self, movie_id: str, title: str, year: int, genres: list):
        self.movie_id = movie_id
        self.title = title
        self.year = year
        self.genres = genres
    
    def __str__(self):
        return f"Movie: {self.movie_id}, {self.title}, {self.year}, {self.genres}"

def get_movie_from_dataset_entity(enity: str):
    parts = enity.split("::")
    if len(parts) < 3:
        return None
    movie_id = parts[0]
    title_and_year = parts[1].split('(')
    title = title_and_year[0]
    year = int(title_and_year[1][:-1])
    genres = parts[2].split("|")
    return Movie(movie_id, title, year, genres)


def get_movies_data(length = None, load_all = False, url = "https://raw.githubusercontent.com/sidooms/MovieTweetings/master/latest/movies.dat"):
    if length:
        print(f"Getting first {length} data")
    else:
        print(f"Getting all data from dataset")
    print(f"Start downloading movies data from https://raw.githubusercontent.com/sidooms/MovieTweetings/master/latest/movies.dat")
    r = requests.get(url)
    movies_str_list = r.content.decode("utf-8").split("\n")
    movies_list = list()
    count = 0
    for item in movies_str_list:
        movie = get_movie_from_dataset_entity(item)
        if movie != None:
            movies_list.append(movie)
        if load_all == False and (count + 1) == length:
            break
        count += 1

    print(f"Got movies_list {len(movies_list)}")
    return movies_list

def fill_movies(cursor: psycopg2.extensions.cursor, movies: list):
    query = '''
        ALTER SEQUENCE movie_genre_id_seq RESTART WITH 1                                
    '''
    cursor.execute(query)
    print("Reset id sequence in movie_genre table")
    movies_query = '''
        INSERT INTO movies (movie_id, title, year) VALUES (%s, %s, %s); 
    '''
    query = '''
        SELECT * FROM genre                                
    '''
    cursor.execute(query)
    genres_fetch = cursor.fetchall()
    genres_dict = dict()
    for genre in genres_fetch:
        genres_dict[genre[1]] = genre[0]

    genre_query = '''
        INSERT INTO movie_genre (movie_id, genre_id) VALUES (%s, %s);
    '''

    print("Filling movies table and movie_genre table")
    for movie in tqdm(movies):
        cursor.execute(movies_query, (movie.movie_id, movie.title, movie.year))
        for genre in movie.genres:
            if genre == '':
                continue
            genre_id = genres_dict[genre] 
            cursor.execute(genre_query, (movie.movie_id, genre_id))
    print("Filled movies and movie_genre table")
