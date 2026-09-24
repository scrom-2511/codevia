from codevia.database.redis.client import RedisDb
import redis

class ApiKeyProvider:
    def __init__(self, redisDb: RedisDb):
        self.api_provider_key = "api_keys"
        self.redisClient: redis.Redis = redisDb.redisClient
        self.api_keys = ["key_1"]

        self._add_api_keys()

    def _add_api_keys(self):
        try:
            self.redisClient.lpush(self.api_provider_key, *self.api_keys)
        except Exception as e:
            print(f"Failed to push keys to Redis: {e}")

    def get_api_key(self) -> str:
        api_key = self.redisClient.lpop(self.api_provider_key)

        if api_key is None:
            raise ValueError("No API keys available")
        
        self.redisClient.lpush(self.api_provider_key, api_key)

        return api_key