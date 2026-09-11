from .utils.tools import Vector
from typing_extensions import Optional
import pinecone

class VectorDB:
    def __init__(self, api_key:Optional[str] = None, host:Optional[str] = None):
        self.api_key = api_key
        self.host = host
        if api_key:
            self.client = pinecone.Pinecone(api_key=api_key, host=host)
        else:
            self.client = pinecone.Pinecone(host=host) # connect to the local docker instance

    def create_index(self, index_name:str, dimension:int = 1024):
        if index_name not in self.client.list_indexes().names():
            self.client.create_index(
                name=index_name,
                dimension=dimension,
                metric="cosine"
            )
        return self.client.Index(index_name)
    
    def upsert_vectors(self, namespace:str, vectors:list[Vector], index_name:str) -> None:
        index = self.client.Index(index_name)
        index.upsert(vectors=vectors, namespace=namespace)

    def search_vectors(self, index_name: str, query_embedding: list[float], top_k: Optional[int] = 5):
        index = self.client.Index(index_name)
        index.query(vector=query_embedding, top_k=top_k, include_metadata=True)
