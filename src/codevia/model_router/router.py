from pydantic import BaseModel, Field
from codevia.llm.client import LLMClient

class TaskScores(BaseModel):
    complexity: int = Field(ge=1, le=10, description="Complexity score (1-10)")
    reasoning: int = Field(ge=1, le=10, description="Reasoning score (1-10)")
    coding: int = Field(ge=1, le=10, description="Coding score (1-10)")
    context: int = Field(ge=1, le=10, description="Context score (1-10)")
    tool_usage: int = Field(ge=1, le=10, description="Tool usage score (1-10)")
    risk: int = Field(ge=1, le=10, description="Risk score (1-10)")

class ModelRouter:
    def __init__(self, llm_client: LLMClient):
        self.llm_client = llm_client
    
    def get_prompt(self, user_task: str) -> str:
        prompt = f"""You are a task-routing classifier for a coding assistant.

        Your job is to analyze the user's coding task and estimate what level of capability is required to complete it successfully.

        Do NOT solve the task.
        Do NOT suggest an implementation.
        Do NOT choose a model.
        Only analyze the task and return the required capability scores.

        Score each dimension from 1 to 10.

        ### Scoring criteria

        **complexity**

        * 1–2: trivial change or straightforward operation
        * 3–4: simple implementation with limited reasoning
        * 5–6: moderate implementation involving multiple steps or files
        * 7–8: complex implementation requiring substantial reasoning or codebase understanding
        * 9–10: highly complex task involving multiple interacting systems, difficult debugging, architecture changes, or significant uncertainty

        **reasoning**

        * 1–2: almost no reasoning required
        * 3–4: straightforward reasoning
        * 5–6: moderate multi-step reasoning
        * 7–8: substantial analysis, debugging, or problem solving
        * 9–10: very difficult multi-step reasoning, deep debugging, concurrency issues, architectural reasoning, or significant uncertainty

        **coding**

        * 1–2: little or no code modification
        * 3–4: simple localized code change
        * 5–6: moderate implementation
        * 7–8: substantial implementation involving multiple components
        * 9–10: sophisticated implementation requiring advanced programming knowledge

        **context**

        * 1–2: task can be completed from a small, clearly identified piece of code
        * 3–4: requires understanding a few related files
        * 5–6: requires understanding several parts of the codebase
        * 7–8: requires broad codebase exploration or understanding of multiple components
        * 9–10: requires understanding a large portion of the architecture or interactions between many systems

        **tool_usage**

        * 1–2: little or no tool usage required
        * 3–4: basic file search/read/edit operations
        * 5–6: normal coding-agent workflow involving search, inspection, modification, and testing
        * 7–8: extensive exploration and multiple tool interactions are likely required
        * 9–10: sophisticated multi-step tool usage, extensive debugging, testing, environment inspection, or interaction with external systems

        **risk**

        * 1–2: harmless/local change where mistakes are easy to recover from
        * 3–4: limited impact if implemented incorrectly
        * 5–6: could break part of the application
        * 7–8: significant functionality or infrastructure could be affected
        * 9–10: mistakes could cause severe data loss, security problems, production outages, or other major consequences

        ### Important instructions

        Evaluate the actual task rather than relying on keywords.

        For example, the presence of the word "debug" does NOT automatically mean high reasoning.

        A trivial debugging task may have:

        reasoning: 2

        while a difficult concurrency bug may have:

        reasoning: 10

        Likewise, a task mentioning many files does not automatically require high context. Estimate what would actually be necessary to solve the task.

        If information is missing, estimate based on the task as stated rather than inventing details.

        User task:

        {user_task}
        """
        return prompt

    def get_scores(self, user_task: str) -> TaskScores:
        prompt = self.get_prompt(user_task)
        
        response = self.llm_client.generate_response(
            query=prompt,
            response_schema=TaskScores.model_json_schema()
        )
        
        return TaskScores.model_validate_json(response.output_text)