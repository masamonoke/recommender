from datetime import datetime
import random 
from utils import weighted_sample

class Rating:
    def __init__(self, 
                 user_id: str,
                 movie_id: str, 
                 rating: int, 
                 rating_timestamp: datetime, 
                 rating_type: str):
        self.user_id = user_id
        self.movie_id = movie_id
        self.rating = rating
        self.rating_timestamp = rating_timestamp
        self.rating_type = rating_type

    def __str__(self):
        return f"Rating: {self.user_id}, {self.movie_id}, {self.rating}, {self.rating_timestamp}, {self.rating_type}"

class Movie:
    def __init__(self, movie_id: str, title: str, year: int, genres: list):
        self.movie_id = movie_id
        self.title = title
        self.year = year
        self.genres = genres
    
    def __str__(self):
        return f"Movie: {self.movie_id}, {self.title}, {self.year}, {self.genres}"

class User:
    def __init__(self, user_id, action_ratio, drama_ratio, comedy_ratio):
        self.session_id = random.randint(0, 1000000)
        self.user_id = user_id
        self.likes = { "action": action_ratio, "drama": drama_ratio, "comedy": comedy_ratio }
        self.events = { self.session_id: [] }

    def get_session_id(self):
        if random.randint(0, 100) > 90:
            self.session_id += 1
            self.events[self.session_id] = []
        return self.session_id

    def select_genre(self):
        return weighted_sample(self.likes)

class EvidenceLog:
    def __init__(self, user_id, content_id, event, session_id, created_timestamp):
        self.user_id = user_id
        self.content_id = content_id
        self.event = event
        self.session_id = session_id
        self.created_timestamp = created_timestamp
    
    def __str__(self):
        return f"Log: {self.user_id}, {self.content_id}, {self.event}, {self.session_id}, {self.created_timestamp}"

