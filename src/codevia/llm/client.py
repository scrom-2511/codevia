from codevia.key_provider.api_key_provider import ApiKeyProvider
from google import genai
from dotenv import load_dotenv
import os

load_dotenv()

class LLMClient:
    def __init__(self, api_keys_provider: ApiKeyProvider):
        # self.client = genai.Client(api_key=os.getenv("GEMINI_API_KEY")) <---- use this in prod
        self.api_keys_provider = api_keys_provider

    def get_prompt(self, query: str, context: str) -> str:
            return f"""
            You are a codebase assistant.

            Answer the user's question using the provided code context.

            User question:
            {query}

            Code context:
            {context}

            Instructions:
            - Answer based on the provided code.
            - Mention relevant file paths.
            - If the context does not contain enough information, say so.
            """.strip()
        
    def generate_response(self, query: str, context:str) -> dict:
        prompt = self.get_prompt(query, context)
        client = genai.Client(api_key=self.api_keys_provider.get_api_key())

        response = client.interactions.create(
            model=os.getenv("GEMINI_MODEL"),
            input=prompt
        )
        
        return {"output_text": response.output_text, "previous_interaction_id": response.previous_interaction_id}

    def generate_stream_response(self, query: str, context:str):
        prompt = self.get_prompt(query, context)
        client = genai.Client(api_key=self.api_keys_provider.get_api_key())

        stream = client.interactions.create(
            model=os.getenv("GEMINI_MODEL"),
            input=prompt,
            stream=True
        )
        
        for event in stream:
            if event.event_type == "step.delta":
                if event.delta.type == "text":
                    print(event.delta.text, end="") # Use yield if another a frontend stream senders calls it