import redis

class RedisDb:
    def __init__(self):
        self.redisClient = redis.Redis(host='localhost', port=6379, decode_responses=True)