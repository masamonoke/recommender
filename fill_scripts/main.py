from fill_movies import *
from fill_ratings import *
from dotenv import load_dotenv
import argparse
import sys

def main():
    load_dotenv()
    parser = argparse.ArgumentParser()
    parser.add_argument("-l", "--len", help="Load first len entities from dataset")
    parser.add_argument("-a", action="store_true")
    args = parser.parse_args()

    movies = get_movies_data(length=int(args.len), load_all=args.a) if args.len != None else get_movies_data(length=None)
    movies_dict = dict()
    for movie in movies:
        movies_dict[movie.movie_id] = movie
    
    ratings = get_ratings_list(movies_dict)
    
    try:
        connection = get_connection()
        cursor = connection.cursor()
        cursor.execute("SELECT version();")
        record = cursor.fetchone()
        print("You are connected to - ", record, "\n")

        truncate("movies", cursor)
        fill_movies(cursor, movies) 

        truncate("ratings", cursor)
        fill_ratings(ratings, cursor)

    except Exception as error:
        print("Error while connecting to PostgreSQL", error)
    
    finally:
        if (connection):
            cursor.close()
            connection.close()
            print("PostgreSQL connection is closed")

if __name__ == "__main__":
    main()

