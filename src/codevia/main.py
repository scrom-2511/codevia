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

    print(tool_map)

    print("Codevia LLM Agent. Type 'exit' to quit.")

    while True:
        try:
            user_input = input("\nYou: ")
            if user_input.lower() in ['exit', 'quit']:
                break
                
            conversation.add_message(conversation_id, {"role": "user", "content": user_input})
            
            selected_model = model_router.route_task(user_input).value
            print(f"Routed task to: {selected_model}")
            
            res = llm.generate_response(
                query=user_input,
                tools=tools_list,
                history=conversation.get_history(conversation_id),
                model=selected_model
            )
            interaction = res.response
            
            while True:
                function_called = False
                function_calls_made = []
                function_responses = []

                for step in interaction.steps:
                    if step.type == "function_call":
                        function_called = True
                        function_calls_made.append({"name": step.name, "arguments": step.arguments})
                        # print(f"Function to call: {step.name}")
                        # print(f"Arguments: {step.arguments}")
                        
                        func = tool_map.get(step.name)
                        if func:
                            try:
                                result = func(**step.arguments)
                            except Exception as e:
                                result = str(e)
                            # print(f"Function result: {result}")
                            
                            function_responses.append({
                                "name": step.name,
                                "response": {"result": result}
                            })
                        else:
                            # print(f"Tool {step.name} not found.")
                            function_responses.append({
                                "name": step.name,
                                "response": {"error": "Tool not found"}
                            })
                
                if function_called:
                    call_msg = f"Model called functions: {json.dumps(function_calls_made)}"
                    conversation.add_message(conversation_id, {"role": "model", "content": call_msg})

                    tool_msg = f"Tool execution results: {json.dumps(function_responses)}"
                    conversation.add_message(conversation_id, {"role": "user", "content": tool_msg})
                    print("model used:", selected_model)
                    
                    res = llm.generate_response(
                        query="Please continue based on the tool results.",
                        tools=tools_list,
                        history=conversation.get_history(conversation_id),
                        model=selected_model
                    )
                    interaction = res.response
                else:
                    print(f"\nLLM: {interaction.output_text}")
                    conversation.add_message(conversation_id, {"role": "model", "content": interaction.output_text})
                    break

        except KeyboardInterrupt:
            break
        except Exception as e:
            print(f"Error: {e}")

if __name__ == "__main__":
    main()