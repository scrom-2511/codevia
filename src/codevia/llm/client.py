from google import genai
from dotenv import load_dotenv
import os

load_dotenv()

class LLMClient:
    def __init__(self):
        self.client = genai.Client(api_key=os.getenv("GEMINI_API_KEY"))
        
    def generate_response(self, prompt: str) -> dict:
        response = self.client.interactions.create(
            model=os.getenv("GEMINI_MODEL"),
            input=prompt
        )
        
        return {"output_text": response.output_text, "previous_interaction_id": response.previous_interaction_id}

    def generate_stream_response(self, prompt: str):
        stream = self.client.interactions.create(
            model=os.getenv("GEMINI_MODEL"),
            input=prompt,
            stream=True
        )
        
        for event in stream:
            if event.event_type == "step.delta":
                if event.delta.type == "text":
                    print(event.delta.text, end="") # Use yield if another a frontend stream senders calls it