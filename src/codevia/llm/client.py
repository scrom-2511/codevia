from typing import Optional, Any
from dataclasses import dataclass
from codevia.key_provider.api_key_provider import ApiKeyProvider
from google import genai
from dotenv import load_dotenv
import os
import inspect

load_dotenv()

@dataclass
class LLMResponse:
    output_text: str
    previous_interaction_id: Optional[str]
    response: Any

class LLMClient:
    def __init__(self, api_keys_provider: ApiKeyProvider):
        self.client = genai.Client(api_key=os.getenv("GEMINI_API_KEY"))
        # self.api_keys_provider = api_keys_provider

    def get_prompt(
        self,
        query: str,
        history: list[dict] | None = None,
        code_context: str | None = None,
    ) -> str:
        history_section = ""
        if history:
            turns = [
                f"{msg.get('role', 'unknown').capitalize()}: {msg.get('content', '')}"
                for msg in history
            ]
            history_section = (
                "### Conversation History\n" + "\n".join(turns) + "\n\n"
            )

        code_section = ""
        if code_context:
            code_section = f"### Code Context\n{code_context}\n\n"

        return inspect.cleandoc(f"""
            You are a codebase assistant. Answer the user's question using the provided context and conversation history.

            ### Instructions
            - Answer based on the provided code context.
            - Mention relevant file paths.
            - If the context does not contain enough information, state that clearly.
            - You have access to tools. If a tool can help you answer the user's question or perform a requested action, call the appropriate tool.

            {code_section}{history_section}### Current Question
            {query}
        """)
        
    def generate_response(self, query: str, tools: Optional[list] = None, history: Optional[list] = None, response_schema: dict = None) -> LLMResponse:
        prompt = self.get_prompt(query, history)
        # client = genai.Client(api_key=self.api_keys_provider.get_api_key())

        response_format = None

        if response_schema is not None:
            response_format = {
                "type": "text",
                "mime_type": "application/json",
                "schema": response_schema,
            }

        response = self.client.interactions.create(
            model=os.getenv("GEMINI_MODEL"),
            input=prompt,
            tools=tools,
            response_format=response_format,
        )
        
        return LLMResponse(
            output_text=response.output_text, 
            previous_interaction_id=response.previous_interaction_id, 
            response=response
        )

    def generate_stream_response(self, query: str, context:str):
        prompt = self.get_prompt(query, context)
        # client = genai.Client(api_key=self.api_keys_provider.get_api_key())

        stream = self.client.interactions.create(
            model=os.getenv("GEMINI_MODEL"),
            input=prompt,
            stream=True
        )
        
        for event in stream:
            if event.event_type == "step.delta":
                if event.delta.type == "text":
                    print(event.delta.text, end="") # Use yield if another a frontend stream senders calls it