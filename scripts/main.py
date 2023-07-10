from fill_movies import *
from fill_explicit_ratings import *
from fill_gen_evidence import fill_evidence_logs
from dotenv import load_dotenv
import argparse
import sys

NUMBER_OF_EVENTS = 10000

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

        truncate("evidence_log", cursor)
        fill_evidence_logs(int(args.elen), cursor) if args.elen != None else fill_evidence_logs(NUMBER_OF_EVENTS, cursor)

    except Exception as error:
        print("Error while connecting to PostgreSQL", error)
    
    finally:
        if (connection):
            cursor.close()
            connection.close()
            print("PostgreSQL connection is closed")

if __name__ == "__main__":
    main()

