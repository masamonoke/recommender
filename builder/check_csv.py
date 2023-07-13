from utils import read_csv_to_dict, get_connection
from dotenv import load_dotenv

load_dotenv()
data = read_csv_to_dict()
conn = get_connection()
cursor = conn.cursor()
no_film = list()
query = '''
    select title from movies where movie_id = %s
'''
for k, films_id in data.items():
    for film in films_id:
        cursor.execute(query, (film, ))
        res = cursor.fetchall()
        if len(res) == 0:
            print(f"There is no film with id {film}")
            no_film.append(film)

if len(no_film) == 0:
    print("All films from csv are in database")
