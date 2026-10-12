from codevia.llm.client import LLMClient

client = LLMClient(api_keys_provider=None)

tools = [{"type": "function", "name": "get_name", "description": "get the name of the user", "parameters": {"type": "object", "properties": {}}}]

clientResponse = client.generate_stream_response(query="how are you mate? and what is my name?", tools=tools)

def get_name():
    print("get_name")
    return "codevia"

for event in clientResponse:
    if event.event_type == "interaction.created":
        first_interaction_id = event.interaction.id
    elif event.event_type == "step.start":
        step = event.step
        if step.type == "function_call":
            func_call_id = step.id
            func_call_name = step.name
            print(func_call_id)
            print(func_call_name)
    elif event.event_type == "step.delta":
        if event.delta.type == "arguments_delta":
            func_args_accumulated += event.delta.arguments
