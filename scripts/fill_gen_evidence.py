import csv
import random
from datetime import datetime
from tqdm import tqdm
from models import User, EvidenceLog
from utils import weighted_sample

def read_csv_to_dict(f = "films.csv"):
    films = dict()

    with open("films.csv", "r") as file:
        reader = csv.reader(file)
        first_row = False
        for row in reader:
            if not first_row:
                first_row = True
                genres = row[0].split(";")
                for genre in genres:
                    films[genre] = list()
            else:
                items = row[0].split(";")
                for genre, item in zip(films.keys(), items):
                    films[genre].append(item)

    return films

def select_film(user, films):
    genre = user.select_genre()
    interested_films = films[genre]
    film_id = ""
    while film_id == "":
        candidate = interested_films[random.randint(0, len(interested_films) - 1)]
        if candidate not in user.events[user.session_id]:
            film_id = candidate

    return film_id

ACTIONS_WEIGHTS = {'genreView': 15, 'details': 50, 'moreDetails': 24, 'addToList': 10, 'buy': 1}

def select_action(user):
    return weighted_sample(ACTIONS_WEIGHTS)

def save_logs(logs, cursor):
    query = '''
        ALTER SEQUENCE evidence_log_id_seq RESTART WITH 1                                
    '''
    cursor.execute(query)
    print("Reset id sequence in evidence_log table")
    query = '''
        INSERT INTO evidence_log (created, content_id, event, session_id, user_id)
        VALUES (%s, %s, %s, %s, %s);
    '''
    print("Filling evidence_log table")
    for log in tqdm(logs):
        cursor.execute(query, 
            (log.created_timestamp, log.content_id, log.event, log.session_id, log.user_id))
    
    print("Filled evidence_log table")

def fill_evidence_logs(length, cursor):
    films = read_csv_to_dict()
    # several personas with tastes expressed in likes ratio
    users = [
        User(2, 20, 30, 50),
        User(3, 50, 20, 40),
        User(4, 20, 30, 50),
        User(5, 100, 0, 0),
        User(6, 0, 100, 0),
        User(7, 0, 0, 100)
    ]
   
    logs = list()

    for _ in range(0, length):
        random_user_id = random.randint(0, len(users) - 1)
        user = users[random_user_id]
        selected_film = select_film(user, films)
        action = select_action(user)
        if action == "buy":
            user.events[user.session_id].append(selected_film)

        log = EvidenceLog(user.user_id, selected_film, action, user.get_session_id(), datetime.now().strftime("%Y-%m-%d %H:%M:%S"))
        logs.append(log)

    save_logs(logs, cursor)


if __name__ == "__main__":
    fill_evidence_logs()
