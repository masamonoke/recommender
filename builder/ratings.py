import requests
import sys
import time
import psycopg2

from datetime import datetime, date
from utils import get_connection, truncate
from tqdm import tqdm
from dotenv import load_dotenv
from users import get_all_users_data_from_db

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
            print(f"Rating data with not enough values found: {parts}")
    return ratings_list

def fill_ratings(ratings: list, cursor: psycopg2.extensions.cursor):
    if len(ratings) == 0:
        print(f"There is no ratings data to insert")
        return
    query = '''
        INSERT INTO ratings (user_id, movie_id, rating, rating_timestamp, rating_type) VALUES(%s, %s, %s, %s, %s);
    '''
    print(f"Filling ratings table with ratings")
    for rating in tqdm(ratings):
        cursor.execute(query,
            (rating.user_id, rating.movie_id, rating.rating, rating.rating_timestamp, rating.rating_type))

    print(f"Filled ratings table with ratings")

# buy weight
w1 = 100
# details weight
w2 = 50
# moreDetails weight
w3 = 15

def get_user_evidence_data(user_id: str, cursor: psycopg2.extensions.cursor) -> dict:
    query = '''
        SELECT content_id,
           COUNT(CASE WHEN event = 'buy' THEN 1 END) AS buys,
           COUNT(CASE WHEN event = 'details' THEN 1 END) AS details,
           COUNT(CASE WHEN event = 'moreDetails' THEN 1 END) AS moredetails
        FROM evidence_log log
        JOIN movies ON log.content_id = movies.movie_id
        WHERE user_id = %s
        GROUP BY user_id, content_id, movies.title
        ORDER BY buys DESC, details DESC, moredetails DESC;
    '''
    cursor.execute(query, (user_id,))
    user_data = cursor.fetchall()

    if len(user_data) == 0:
        print(f"Not enough evidence data to calculate implicit ratings for user_id={user_id}")
        return None

    d = dict()

    for item in user_data:
        movie_id = item[0]
        buys = item[1]
        details = item[2]
        moredetails = item[3]
        d[movie_id] = dict()
        d[movie_id]["buy"] = buys
        d[movie_id]["details"] = details
        d[movie_id]["moredetails"] = moredetails

    return d

def calculate_implicit_rating(user_id: str, data: dict) -> list:
    ratings = dict()
    # for normalization
    max_rating = 0

    for movie_id, v in data.items():
        rating = w1 * v["buy"] + w2 * v["details"] + w3 * v["moredetails"]
        max_rating = max(rating, max_rating)
        ratings[movie_id] = rating

    r = list()
    for movie_id in ratings.keys():
        ratings[movie_id] = round(10 * ratings[movie_id] / max_rating, 2)
        # timestamp = time.strftime("%Y-%m-%d% %H:%M:%S", time.gmtime(int(parts[3])))
        timestamp = date.today().strftime("%Y-%m-%d% %H:%M:%S")
        r.append(Rating(user_id, movie_id, ratings[movie_id], timestamp, "implicit"))

    return r

def build_explicit_ratings(movies_dict: dict, cursor: psycopg2.extensions.cursor):
    explicit_ratings = get_ratings_list(movies_dict)
    fill_ratings(explicit_ratings, cursor)

def build_implicit_ratings(cursor: psycopg2.extensions.cursor):
    data = get_all_users_data_from_db(cursor)
    user_ids = list()
    for i in data:
        user_ids.append(i[0])
    implicit_ratings = list()
    for id in user_ids:
        user_data = get_user_evidence_data(id, cursor)
        if user_data == None:
            continue
        implicit_ratings.extend(calculate_implicit_rating(id, user_data))
    fill_ratings(implicit_ratings, cursor)
