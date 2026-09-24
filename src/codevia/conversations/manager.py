import json
from uuid import uuid4
from codevia.database.redis.client import RedisDb
import redis

from typing import TypedDict

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