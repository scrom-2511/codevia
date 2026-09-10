from google import genai
from dotenv import load_dotenv
import os

load_dotenv()

class LLMClient:
    def __init__(self):
        self.client = genai.Client(api_key=os.getenv("GEMINI_API_KEY"))
        
    def generate_response(self, prompt: str) -> str:
        response = self.client.models.generate_content(
            model=os.getenv("GEMINI_MODEL"),
            contents=prompt,
        )
        return response.text