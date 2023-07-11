import psycopg2
import random
import string
import uuid

from utils import weighted_sample, truncate

class User:
    def __init__(self, user_id, action_ratio, drama_ratio, comedy_ratio):
        self.session_id = random.randint(0, 1000000)
        self.user_id = user_id
        self.likes = { "action": action_ratio, "drama": drama_ratio, "comedy": comedy_ratio }
        self.events = { self.session_id: [] }
        name = random.choice(names)
        while name in taken_names:
            name = random.choice(names)
        taken_names.add(name)
        self.name = name
        self.email = genEmail()
        self.password= genPassword()
        self.uuid = str(uuid.uuid4())

    def save_to_db(self, cursor: psycopg2.extensions.cursor):
        print(f"Saving user: {self}")
        query = '''
            INSERT INTO users (name, email, password, unique_id)
            VALUES (%s, %s, %s, %s);
        '''
        cursor.execute(query, (self.name, self.email, self.password, self.uuid))


    def get_session_id(self):
        if random.randint(0, 100) > 90:
            self.session_id += 1
            self.events[self.session_id] = []
        return self.session_id

    def select_genre(self):
        return weighted_sample(self.likes)

    def __str__(self):
        return f"User: name={self.name}, email={self.email}, password={self.password}, uuid={self.uuid}"

taken_names = set()

names = ['Oliver', 'George', 'Noah', 'Arthur', 'Harry',
           'Leo', 'Muhammad', 'Jack', 'Charlie', 'Oscar',
           'Jacob', 'Henry', 'Thomas', 'Freddie', 'Alfie',
           'Theo', 'William', 'Theodore', 'Archie', 'Joshua',
           'Alexander', 'James', 'Isaac', 'Edward', 'Lucas',
           'Tommy', 'Finley', 'Max', 'Logan', 'Ethan',
           'Teddy', 'Benjamin', 'Arlo', 'Joseph', 'Sebastian',
           'Harrison', 'Elijah', 'Adam', 'Daniel', 'Samuel',
           'Louie', 'Mason', 'Albie', 'Rory', 'Hugo',
           'Olivia', 'Emma', 'Charlotte', 'Sophia', 'Ava',
           'Evelyn', 'Harper', 'Luna', 'Camila', 'Gianna',
           'Sofia', 'Avery', 'Scarlett', 'Emily', 'Aria',
           'Lilith', 'Penelope', 'Layla', 'Mila', 'Nora',
           'Hazel', 'Ellie', 'Lily', 'Nova', 'Isla',
           'Grace', 'Emilia', 'Stella', 'Zoe', 'Victoria',
           'Zahra', 'Jamie', 'Allyson', 'Anahi', 'Karla ',
           'Johanna', 'Hayley', 'Freyja', 'Florence', 'Bridget',
           'Joelle', 'Roselyn', 'Aila', 'Katelyn', 'Lea', 
]

emails_ending = ['yahoo.com', 'gmail.com', 'yandex.ru', 'mail.ru']

def genPassword():
    p = ''
    for i in range(10):
        p += random.choice(string.ascii_letters)
    return p

def genEmail():
    e = ''
    for i in range(10):
        e += random.choice(string.ascii_letters)
    e += '@' + emails_ending[random.randrange(0, len(emails_ending) - 1)]
    return e

def get_all_users_data_from_db(cursor: psycopg2.extensions.cursor) -> list:
    query = '''
        SELECT * from users;
    '''
    cursor.execute(query)
    return cursor.fetchall()

def save_users(users: list, cursor: psycopg2.extensions.cursor):
    truncate("users", cursor)
    query = '''
        ALTER SEQUENCE users_id_seq RESTART WITH 1; 
    '''
    cursor.execute(query)
    for user in users:
        user.save_to_db(cursor)
