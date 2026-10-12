import json
from pathlib import Path
from codevia.conversations.manager import Conversations
from codevia.llm.client import LLMClient
from codevia.tools.workspace import WorkspaceTools
from codevia.database.redis.client import RedisDb
from codevia.model_router.router import ModelRouter

def load_tools_definition():
    file_path = Path(__file__).parent / "txt_information" / "tools_definition.json"
    with open(file_path, "r", encoding="utf-8") as f:
        return json.load(f)

def main():
    redis_db = RedisDb()
    llm = LLMClient(api_keys_provider=None)
    conversation = Conversations(redisDb=redis_db)
    conversation_id = conversation.create_conversation()

    workspace_tools = WorkspaceTools()
    tools_list = load_tools_definition()
    
    model_router = ModelRouter(llm_client=llm)

    tool_map = {}
    for tool in tools_list:
        func_name = tool.get("name")
        if func_name and hasattr(workspace_tools, func_name):
            tool_map[func_name] = getattr(workspace_tools, func_name)

    # print(tool_map)

    print("Codevia LLM Agent. Type 'exit' to quit.")

    while True:
        try:
            user_input = input("\nYou: ")
            if user_input.lower() in ['exit', 'quit']:
                break
                
            conversation.add_message(conversation_id, {"role": "user", "content": user_input})
            
            selected_model = model_router.route_task(user_input).value
            # print(f"Routed task to: {selected_model}")
            
            res = llm.generate_stream_response(
                query=user_input,
                tools=tools_list,
                history=conversation.get_history(conversation_id),
                model=selected_model
            )
            
            while True:
                function_called = False
                function_calls_made = []
                function_responses = []
                accumulated_text = ""
                current_call = None
                accumulated_args_str = ""

                for event in res:
                    if event.event_type == "step.start":
                        step = event.step
                        if step.type == "function_call":
                            current_call = {
                                "id": step.id or None,
                                "name": step.name
                            }
                            accumulated_args_str = ""

                    elif event.event_type == "step.delta":
                        if event.delta.type == "text":
                            accumulated_text += event.delta.text
                            print(event.delta.text, end="", flush=True)
                        elif event.delta.type == "arguments_delta":
                            accumulated_args_str += event.delta.arguments or ""

                    elif event.event_type == "step.stop":
                        if current_call:
                            function_called = True
                            try:
                                args = json.loads(accumulated_args_str) if accumulated_args_str else {}
                            except json.JSONDecodeError:
                                args = {}

                            function_calls_made.append({"name": current_call["name"], "arguments": args})

                            func = tool_map.get(current_call["name"])
                            if func:
                                try:
                                    result = func(**args)
                                    print(result)
                                except Exception as e:
                                    result = str(e)
                                
                                function_responses.append({
                                    "name": current_call["name"],
                                    "response": {"result": result}
                                })
                            else:
                                function_responses.append({
                                    "name": current_call["name"],
                                    "response": {"error": "Tool not found"}
                                })

                            current_call = None
                            accumulated_args_str = ""

                if function_called:
                    print("\nfunction is called")
                    if accumulated_text:
                        conversation.add_message(conversation_id, {"role": "model", "content": accumulated_text})

                    call_msg = f"Model called functions: {json.dumps(function_calls_made)}"
                    conversation.add_message(conversation_id, {"role": "model", "content": call_msg})

                    tool_msg = f"Tool execution results: {json.dumps(function_responses)}"
                    conversation.add_message(conversation_id, {"role": "user", "content": tool_msg})
                    
                    res = llm.generate_stream_response(
                        query="Please continue based on the tool results.",
                        tools=tools_list,
                        history=conversation.get_history(conversation_id),
                        model=selected_model
                    )
                    continue
                else:
                    if accumulated_text:
                        conversation.add_message(conversation_id, {"role": "model", "content": accumulated_text})
                    print("\nModel finished execution.")
                    break
                        

        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"Error: {e}")

if __name__ == "__main__":
    main()