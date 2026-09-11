import voyageai
from dotenv import load_dotenv
import os

load_dotenv()
class Embedder:
    def __init__(self):
        self.client = voyageai.Client(api_key=os.getenv("VOYAGE_API_KEY"))
    
    def embed(self, chunks: list[str]):
        result = self.client.embed(chunks, model=os.getenv("VOYAGE_MODEL"))
        print(result.embeddings)
        print(len(result.embeddings[0]))
        print(result.total_tokens)
        return result.embeddings 


if __name__ == "__main__":
    embedder = Embedder()
    embedder.embed(chunks=["""let user = prisma.findOne(userId)"""])