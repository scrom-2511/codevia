from pinecone import ServerlessSpec
from .utils.tools import Vector
from typing_extensions import Optional
import pinecone

### Abstraction for sdk

# class VectorDB:
#     def __init__(self, api_key:Optional[str] = None, host:Optional[str] = None):
#         self.api_key = api_key
#         self.host = host
#         if api_key:
#             self.client = pinecone.Pinecone(api_key=api_key, host=host)
#         else:
#             self.client = pinecone.Pinecone(host=host) # connect to the local docker instance

#     def create_index(self, index_name:str, dimension:int = 1024):
#         if index_name not in self.client.list_indexes().names():
#             self.client.create_index(
#                 name=index_name,
#                 dimension=dimension,
#                 metric="cosine",
#                 spec=ServerlessSpec(
#                 cloud="aws",
#                 region="us-east-1"
#             ),
#             )
#         return self.client.Index(index_name)
    
#     def upsert_vectors(self, namespace:str, vectors:list[Vector], index_name:str) -> None:
#         index = self.client.Index(index_name)
#         pinecone_vectors = [
#             {
#                 "id": vector.id,
#                 "values": vector.values,
#                 "metadata": {
#                     "path": vector.metadata.path,
#                     "chunk_index": vector.metadata.chunk_index,
#                     "text": vector.metadata.text,
#                 },
#             }
#             for vector in vectors
#         ]
#         index.upsert(vectors=pinecone_vectors, namespace=namespace)

#     def search_vectors(self, index_name: str, query_embedding: list[float],  namespace: str, top_k: Optional[int] = 5):
#         index = self.client.Index(index_name)
#         result = index.query(vector=query_embedding, top_k=top_k, namespace=namespace, include_metadata=True)

#         return result.matches


### Abstraction for local docker

class VectorDB:
    def __init__(self, host: str):
        self.client = pinecone.Pinecone()
        self.index = self.client.Index(host=host)

    def upsert_vectors(self, namespace, vectors):
        pinecone_vectors = [
            {
                "id": vector.id,
                "values": vector.values,
                "metadata": {
                    "path": vector.metadata.path,
                    "chunk_index": vector.metadata.chunk_index,
                    "text": vector.metadata.text,
                },
            }
            for vector in vectors
        ]

        self.index.upsert(
            vectors=pinecone_vectors,
            namespace=namespace,
        )

    def search_vectors(self, namespace, query_embedding, top_k=5):
        result = self.index.query(
            vector=query_embedding,
            top_k=top_k,
            namespace=namespace,
            include_metadata=True,
        )

        return result.matches