from dotenv import load_dotenv
from psycopg2.extensions import cursor
from psycopg2.extras import execute_values
from logging import info
import logging
from datetime import datetime
from tqdm import tqdm

import numpy
import pandas as pd
from scipy.sparse import coo_matrix
from sklearn.metrics.pairwise import cosine_similarity

from utils import get_connection, truncate

# values for filter
MIN_OVERLAP = 20
MIN_SIMILIRAITY = 0.0

def load_ratings(cursor: cursor, min_ratings_count=1) -> pd.DataFrame:
    info("Loading ratings and preparing dataframe")
    query = '''
        SELECT user_id, movie_id, rating, rating_type
        FROM ratings;
    '''
    cursor.execute(query)
    tuples = cursor.fetchall()
    columns = ["user_id", "movie_id", "rating", "type"]
    ratings = pd.DataFrame.from_records(tuples, columns=columns)
    user_movie_count = ratings[["user_id", "movie_id"]].groupby("user_id").count()
    user_movie_count = user_movie_count.reset_index()
    user_ids = user_movie_count[user_movie_count["movie_id"] > min_ratings_count]["user_id"]
    # we keep only those users who rated more than 1 movie
    ratings = ratings[ratings["user_id"].isin(user_ids)]
    ratings["rating"] = ratings["rating"].astype(float)
    return ratings

def build(cursor: cursor, ratings: pd.DataFrame):
    info(f"Start calculating similarity over {len(ratings)} ratings")
    ratings["norm_rating"] = ratings.groupby("user_id")["rating"].transform(lambda x: normalize(x))
    ratings["user_id"] = ratings["user_id"].astype("category")
    ratings["movie_id"] = ratings["movie_id"].astype("category")
    # cat.codes is representation of categorical data as numbers
    coo = coo_matrix( ( ratings["norm_rating"].astype(float), (ratings["movie_id"].cat.codes.copy(), ratings["user_id"].cat.codes.copy()) ) )
    info(f"Rating matrix size: {coo.shape[0]}x{coo.shape[1]}")
    # you can read md/movies_overlap.md for details on that
    overlap_matrix = coo.astype(bool).astype(int).dot(coo.transpose().astype(bool).astype(int))
    corr = cosine_similarity(coo, dense_output=False )
    corr = corr.multiply(corr > MIN_SIMILIRAITY)
    corr = corr.multiply(overlap_matrix > MIN_OVERLAP)
    info(f"cosine similarity matrix shape: {corr.shape}")
    movies = dict(enumerate(ratings["movie_id"].cat.categories))
    save(cursor, corr, movies)

def normalize(x: pd.core.series.Series) -> pd.core.series.Series:
    x = x.astype(float)
    sum = x.sum()
    num = x.astype(bool).sum()
    if num == 1 or x.std() == 0:
        return 0.0
    mean = sum / num
    return (x - mean) / (x.max() - x.min())

def save(cursor: cursor, corr: numpy.ndarray, movie_indicies: dict, timestamp=datetime.now()):
    info("Saving similarities to table 'similarity'")
    sims = list()
    coo = coo_matrix(corr)
    # compressed sparse row matrix
    csr = coo.tocsr()
    truncate("similarity", cursor)
    query = '''
        ALTER SEQUENCE similarity_id_seq RESTART WITH 1;
    '''
    cursor.execute(query)
    query = '''
        INSERT INTO similarity (created, source, target, similarity)
        VALUES %s;
    '''
    info(f"{coo.count_nonzero()} similarities to save")
    xs, ys = coo.nonzero()
    for x, y in tqdm(zip(xs, ys), total=len(xs), leave=False):
        if x == y:
            continue
        sim = csr[x, y]
        if sim < MIN_SIMILIRAITY:
            continue

        new_similarity = (str(timestamp), movie_indicies[x], movie_indicies[y], sim)
        sims.append(new_similarity)
    execute_values(cursor, query, sims)


if __name__ == "__main__":
    load_dotenv()
    logging.basicConfig(level=logging.INFO)
    cursor = get_connection().cursor()
    ratings = load_ratings(cursor)
    build(cursor, ratings)
