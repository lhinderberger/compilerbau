use super::super::lexer::SymbolType;

pub type GraphID = String;
pub type NodeIndex = usize;


pub struct Graph {
    pub id: GraphID,
    pub nodes: Vec<Node>
}

pub struct Node {
    pub vertices: Vec<Vertex>
}

pub struct Vertex {
    pub condition: VertexCondition,
    pub target: VertexTarget
}


pub enum VertexCondition {
    IsIdentifier,
    IsNumber,
    IsSymbol(SymbolType),
    Subgraph(GraphID),
    Nil
}

pub enum VertexTarget {
    EndOfGraph,
    Node(NodeIndex)
}
