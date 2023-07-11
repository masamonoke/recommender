import argparse
import sys

from movies import *
from ratings import build_explicit_ratings, build_implicit_ratings
from evidence_log import fill_evidence_logs
from dotenv import load_dotenv
from users import User, save_users

def main():
    load_dotenv()
    parser = argparse.ArgumentParser()
    parser.add_argument("-m", "--mlen", help="Load first len entities from dataset")
    parser.add_argument("-e", "--elen", help="How much evidence log generated. Default is 10000")
    args = parser.parse_args()

    l = int(args.mlen) if args.mlen != None else None
    if l != None:
        movies = get_movies_data(length=l)
    else:
        movies = get_movies_data(load_all=True)
        
    try:
        connection = get_connection()
        cursor = connection.cursor()
        cursor.execute("SELECT version();")
        record = cursor.fetchone()
        print("You are connected to - ", record, "\n")

        truncate("movies", cursor)
        fill_movies(cursor, movies) 

        truncate("ratings", cursor)
        movies_dict = dict()
        for movie in movies:
            movies_dict[movie.movie_id] = movie
        build_explicit_ratings(movies_dict, cursor)

        # several personas with tastes expressed in likes ratio
        users = [
            User(1, 20, 30, 50),
            User(2, 50, 20, 40),
            User(3, 20, 30, 50),
            User(4, 100, 0, 0),
            User(5, 0, 100, 0),
            User(6, 0, 0, 100)
        ]
        save_users(users, cursor)
        truncate("evidence_log", cursor)
        fill_evidence_logs(users, cursor, int(args.elen)) if args.elen != None else fill_evidence_logs(users, cursor)
        build_implicit_ratings(cursor)

    except psycopg2.Error as error:
        print("Pyscopg error: ", error)

    finally:
        if (connection):
            cursor.close()
            connection.close()
            print("Connection closed")

if __name__ == "__main__":
    main()

