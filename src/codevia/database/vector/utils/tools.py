from dataclasses import dataclass

@dataclass
class Metadata:
    path: str
    chunk_index: int
    text: str

@dataclass
class Vector:
    id: str
    values: list[float]
    metadata: Metadata

def make_vectors(vectors:list[list[float]], metadatas:list[Metadata]) -> list[Vector]:
    if len(vectors) != len(metadatas):
        raise ValueError("vectors and metadatas must have the same length")
        
    result = []
    for vector, metadata in zip(vectors, metadatas):
        result.append(Vector(
            id=f"{metadata.path}:{metadata.chunk_index}",
            values=vector,
            metadata=metadata
        ))
    return result