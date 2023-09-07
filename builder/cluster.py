import psycopg2
from dotenv import load_dotenv
import numpy as np
from tqdm import tqdm
from scipy.sparse import dok_matrix
from sklearn.cluster import KMeans
from time import time

from utils import get_connection, truncate

class Cluster:
    def __init__(self, cluster_id: int, user_id: int):
        self.cluster_id = cluster_id
        self.user_id = user_id

def calculate(cursor: psycopg2.extensions.cursor, k: int = 23):
    all_start = time()
    print("loading user ratings data from 'ratings' table...")
    start_time = time()
    # this one takes the most of execution time
    # on macbook air m1 it took around 30 min to load and build dok matrix
    user_ids, user_ratings = load_data(cursor)
    print(f"loading took {(time() - start_time) / 60} minutes to execute")

    kmeans = KMeans(n_clusters=k)
    print("training k-means model...")
    start_time = time()
    clusters = kmeans.fit(user_ratings.tocsr())
    print(f"model learning took {(time() - start_time) / 60} minutes to execute")

    print("saving learned clusters to table 'clusters'...")
    start_time = time()
    save_clusters(clusters, user_ids, cursor)
    print(f"saving took {time() - start_time} seconds to execute")
    print("finished")
    print(f"all calculation took {(time() - all_start) / 60} minutes to execute")

# TODO: replace queries with code from ratings.py module
def load_data(cursor: psycopg2.extensions.cursor) -> (list, dok_matrix):
    query = '''
        SELECT user_id, count(movie_id) AS c FROM ratings
        GROUP BY user_id
        ORDER BY c DESC;
    '''
    cursor.execute(query)
    tuples = cursor.fetchall()
    user_ids = list()
    for t in tuples:
        user_ids.append(t[0])

    query = '''
        select distinct movie_id
        from ratings;
    '''
    cursor.execute(query)
    tuples = cursor.fetchall()
    content_ids = list()
    for t in tuples:
        content_ids.append(t[0])
    content_map = { content_ids[i]: i for i in range(len(content_ids)) }
    num_users = len(user_ids)
    # dictionary of keys matrix with rows=num_users and cols=len(content_ids)
    # where key is row and col, value is element of matrix[row][col] from key
    user_ratings = dok_matrix((num_users, len(content_ids)), dtype=np.float32)

    for i in tqdm(range(num_users), leave=False, position=0):
        query = '''
            SELECT movie_id, rating
            FROM ratings
            WHERE user_id=%s
        '''
        cursor.execute(query, (user_ids[i], ))
        tuples = cursor.fetchall()
        # list of tuples (movie_id, np.float32 rating value)
        ratings = list()
        for t in tuples:
            f = np.float32(t[1])
            ratings.append((t[0], f))
        for rating in ratings:
            movie_id = rating[0]
            rating_value = rating[1]
            user_ratings[i, content_map[movie_id]] = rating_value
    return user_ids, user_ratings

def save_clusters(clusters: object, user_ids: list, cursor: psycopg2.extensions.cursor):
    truncate("clusters", cursor)
    query = '''
        ALTER SEQUENCE clusters_id_seq RESTART WITH 1;
    '''
    cursor.execute(query)

    print("saving clusters to database")
    query = '''
        INSERT INTO clusters (cluster_id, user_id) VALUES (%s, %s);
    '''
    for i, cluster_label in (enumerate(tqdm(clusters.labels_, leave=False, position=0))):
        cluster = Cluster(cluster_label.item(), user_ids[i])
        cursor.execute(query, (cluster.cluster_id, cluster.user_id))

if __name__ == "__main__":
    load_dotenv()
    connection = get_connection()
    cursor = connection.cursor()
    calculate(cursor)

