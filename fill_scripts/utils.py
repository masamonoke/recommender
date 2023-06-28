import psycopg2
import os
from urllib.parse import urlparse

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

def truncate(name: str, cursor: psycopg2.extensions.cursor):
    query = f'''
        TRUNCATE TABLE {name} CASCADE;
    '''
    cursor.execute(query)
    print(f"{name} table successfully truncated")

