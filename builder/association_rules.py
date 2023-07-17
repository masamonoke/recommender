from psycopg2 import extensions
from utils import get_connection, truncate
from dotenv import load_dotenv
from evidence_log import EvidenceLog
from collections import defaultdict
from itertools import combinations
from datetime import datetime

class Rule:
    def __init__(self, created: datetime, source_id: str, target_id: str, support: float, confidence: float):
        self.created = created
        self.source_id = source_id
        self.target_id = target_id
        self.support = support
        self.confidence = confidence

    def __eq__(self, other):
        return self.created == other.created
    
    def __gt__(self, other):
        return self.created > other.created

    def __lt__(self, other):
        return self.created < other.created

    def __str__(self):
        return f"Rule: created={self.created}, source_id={self.source_id}, target_id={self.target_id}, support={self.support}, confidence={self.confidence}"

def get_buy_events(cursor: extensions.cursor) -> list:
    query = '''
        SELECT *
        FROM evidence_log
        WHERE event = 'buy'
        ORDER BY session_id, content_id;
    '''
    cursor.execute(query)
    tuples = cursor.fetchall()
    logs = list()
    for t in tuples:
        log = EvidenceLog(user_id=t[5], content_id=t[2], event=t[3], session_id=t[4], created_timestamp=t[1])
        logs.append(log)
    
    return logs

def transactions_per_session(logs: list) -> dict:
    transactions = dict()
    for log in logs:
        if log.session_id not in transactions:
            transactions[log.session_id] = list()
        transactions[log.session_id].append(log.content_id)

    return transactions

def calculate_itemsets_one(transactions: dict, min_support=0.01) -> dict:
    n = len(transactions)
    item_occurences = defaultdict(int)
    one_itemsets = dict()

    for session, items in transactions.items():
        for item in items:
            dest = frozenset({item})
            item_occurences[dest] += 1
    
    for item, count in item_occurences.items():
        if count > min_support * n:
            one_itemsets[item] = count

    return one_itemsets

def calculate_itemsets_two(transactions: dict, one_itemsets: dict, min_support=0.01) -> dict:
    two_itemsets = defaultdict(int)
    for session, items in transactions.items():
        non_dublicate_items = list(set(items))
        if len(items) > 2:
            for combo in combinations(non_dublicate_items, 2):
                if support(combo, one_itemsets):
                    two_itemsets[frozenset(combo)] += 1
        elif len(items) == 2:
            if support(items, one_itemsets):
                two_itemsets[frozenset(items)] += 1

    return two_itemsets

def support(combination: set, one_itemsets: dict) -> bool:
    return frozenset({combination[0]}) in one_itemsets and frozenset({combination[1]}) in one_itemsets

def calculate_association_rules(one_itemsets: dict, two_itemsets: dict, length: int) -> list:
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    rules = list()
    for source, source_freq in one_itemsets.items():
        for group, group_freq in two_itemsets.items():
            if source.issubset(group):
                target = group.difference(source)
                support = group_freq / length
                confidence = group_freq / source_freq
                rules.append(Rule(timestamp, next(iter(source)), next(iter(target)), support, confidence))

    return rules

def calculate_support_confidence(transactions: dict, min_support=0.01):
    n = len(transactions)
    one_itemsets = calculate_itemsets_one(transactions, min_support)
    two_itemsets = calculate_itemsets_two(transactions, one_itemsets)
    rules = calculate_association_rules(one_itemsets, two_itemsets, n)
    return sorted(rules)

def save_to_db(rules: list, cursor: extensions.cursor):
    query = '''
        ALTER SEQUENCE seeded_recs_id_seq RESTART WITH 1                                
    '''
    cursor.execute(query)
    query = '''
        INSERT INTO seeded_recs (created, source, target, support, confidence)
        VALUES (%s, %s, %s, %s, %s);
    '''
    for rule in rules:
        values = (rule.created, rule.source_id, rule.target_id, rule.support, rule.confidence)
        cursor.execute(query, values)

def build_seeded_recs(cursor: extensions.cursor):
    logs = get_buy_events(cursor)
    t = transactions_per_session(logs)
    rules = calculate_support_confidence(t)
    print("Truncating seeded_recs table")
    truncate("seeded_recs", cursor)
    print("Saving to seeded_recs table rules values")
    save_to_db(rules, cursor)

if __name__ == "__main__":
    load_dotenv()
    conn = get_connection()
    cursor = conn.cursor()
    build_seeded_recs(cursor)
