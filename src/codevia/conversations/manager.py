from enum import Enum
import json
from uuid import uuid4
from codevia.database.redis.client import RedisDb
import redis
from typing import TypedDict

class Role(Enum):
    USER = "user"
    MODEL = "model"

class Conversation(TypedDict):
    role: str
    content: str

class Conversations:
    def __init__(self, redisDb: RedisDb):
        self.redisClient:redis.Redis = redisDb.redisClient
        self.redis_base_key = "conversations"

    def create_conversation(self):
        conversation_id = uuid4()
        redis_key = f"{self.redis_base_key}:{conversation_id}"

        self.redisClient.set(redis_key, "initialized")

        return conversation_id

    def add_message(self, conversation_id: str, conversation: Conversation) -> None:
        redis_key = f"{self.redis_base_key}:{conversation_id}"

        raw_data = json.dumps(conversation)

        try:
            self.redisClient.rpush(redis_key, raw_data)
        except Exception as e:
            print(f"Error adding message to redis: {e}")

    def get_history(self, conversation_id: str) -> list[Conversation]:
        redis_key = f"{self.redis_base_key}:{conversation_id}"

        try:
            raw_history = self.redisClient.lrange(redis_key, 0, -1)
            conversation_history: list[Conversation] = [json.loads(conversation) for conversation in raw_history]
            return conversation_history

        except Exception as e:
            print(f"Error retrieving history from redis: {e}")
            return []