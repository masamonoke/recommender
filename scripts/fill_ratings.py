import requests
import sys
from datetime import datetime
from utils import get_connection, truncate
import time
from tqdm import tqdm

class Rating:
    def __init__(self, 
                 user_id: str,
                 movie_id: str, 
                 rating: int, 
                 rating_timestamp: datetime, 
                 rating_type: str):
        self.user_id = user_id
        self.movie_id = movie_id
        self.rating = rating
        self.rating_timestamp = rating_timestamp
        self.rating_type = rating_type

    def __str__(self):
        return f"Rating: {self.user_id}, {self.movie_id}, {self.rating}, {self.rating_timestamp}, {self.rating_type}"

def get_ratings_list(movies_dict: dict, url = "https://raw.githubusercontent.com/sidooms/movietweetings/master/latest/ratings.dat"):
    print(f"Start downloading data from {url}")
    r = requests.get(url)
    ratings_str = r.content.decode("utf-8").split("\n")
    ratings_list = list()
    for item in ratings_str:
        parts = item.split("::")
        if len(parts) == 4:
            user_id = parts[0]
            movie_id = parts[1]
            rating_value = parts[2]
            # format example 2013-10-05 21:00:50
            timestamp = time.strftime("%Y-%m-%d% %H:%M:%S", time.gmtime(int(parts[3])))
            if movie_id in movies_dict:
                ratings_list.append(Rating(user_id, movie_id, rating_value, timestamp, "explicit"))
        else:
            print(f"Rating data with not enough values fixated: {parts}")
    return ratings_list

def fill_ratings(ratings, cursor):
    query = '''
        ALTER SEQUENCE ratings_id_seq RESTART WITH 1                                
    '''
    cursor.execute(query)
    print("Reset id sequence in ratings table")
    query = '''
        INSERT INTO ratings (user_id, movie_id, rating, rating_timestamp, rating_type) VALUES(%s, %s, %s, %s, %s);
    '''
    print("Filling ratings table")
    for rating in tqdm(ratings):
        cursor.execute(query, 
            (rating.user_id, rating.movie_id, rating.rating, rating.rating_timestamp, rating.rating_type))
    
    print("Filled ratings table")
