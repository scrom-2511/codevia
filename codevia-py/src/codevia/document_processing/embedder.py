import time
from codevia.database.vector.client import VectorDB
from codevia.database.vector.utils.tools import make_vectors
import voyageai
from dotenv import load_dotenv
import os

load_dotenv()
class Embedder:
    def __init__(self):
        self.client = voyageai.Client(api_key=os.getenv("VOYAGE_API_KEY"))
        self.BATCH_SIZE = 10 # for api limits
        self.WAIT_TIME = 25 # for api limits
    
    def embed(self, chunks: list[str]) -> list[list[float]]:
        result = self.client.embed(chunks, model=os.getenv("VOYAGE_MODEL"))
        return result.embeddings

    def batch_embed(self, chunks: list[str]):
        for start in range(0, len(chunks), self.BATCH_SIZE):
            batch_chunks = chunks[start:start + self.BATCH_SIZE]

            print(
                f"Embedding chunks "
                f"{start + 1}-{start + len(batch_chunks)} "
                f"of {len(chunks)}"
            )

            embeddings = self.embed(
                batch_chunks,
            )

            yield embeddings

            print("Batch uploaded.")

            if start + self.BATCH_SIZE < len(chunks):
                print(f"Waiting {self.WAIT_TIME}s...")
                time.sleep(self.WAIT_TIME)
                print("25 seconds over")


if __name__ == "__main__":
    embedder = Embedder()
    embedder.embed(chunks=["""let user = prisma.findOne(userId)"""])