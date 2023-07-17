import random
from datetime import datetime
from tqdm import tqdm
from users import User
from utils import weighted_sample
import psycopg2
from utils import truncate, read_csv_to_dict

class EvidenceLog:
    def __init__(self, user_id, content_id, event, session_id, created_timestamp):
        self.user_id = user_id
        self.content_id = content_id
        self.event = event
        self.session_id = session_id
        self.created_timestamp = created_timestamp
    
    def __str__(self):
        return f"Log: {self.user_id}, {self.content_id}, {self.event}, {self.session_id}, {self.created_timestamp}"

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

def fill_evidence_logs(users: list, cursor, length: int = 100000):
    films = read_csv_to_dict()

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
