use super::super::lexer::{ Morpheme, MorphemeContent, SymbolType };

pub type NodeIndex = usize;
pub type VertexIndex = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GraphID {
    PROGRAM,
    BLOCK,
        CONST, VAR, PROCEDURE,
    STATEMENT,
        ASSIGNMENT, IF, WHILE, COMPOUND,
        CALL, INPUT, OUTPUT,
    CONDITION,
    EXPRESSION,
        TERM, FACTOR
}


pub struct Graph {
    pub nodes: Vec<Node>
}

pub struct Node {
    pub vertices: Vec<Vertex>
}

pub struct Vertex {
    pub condition: VertexCondition,
    pub target: VertexTarget
}


#[derive(Debug)]
pub struct GraphLocation {
    pub graph: GraphID,
    pub node: NodeIndex
}


#[derive(Debug, PartialEq)]
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


impl VertexCondition {
    pub fn met_for(&self, morpheme: &MorphemeContent) -> bool {
        match self {
            VertexCondition::Nil => true,
            VertexCondition::Subgraph(_) => true,
            _ => match morpheme {
                MorphemeContent::Invalid => false,
                MorphemeContent::Number(_) => self == &VertexCondition::IsNumber,
                MorphemeContent::Identifier(_) => self == &VertexCondition::IsIdentifier,
                MorphemeContent::Symbol(s) => self == &VertexCondition::IsSymbol(*s)
            }
        }
    }

    pub fn met_for_morpheme_or_eof(&self, morpheme_or_eof: &Option<Morpheme>) -> bool {
        match morpheme_or_eof {
            None => match self {
                VertexCondition::Nil => true,
                VertexCondition::Subgraph(_) => true,
                _ => false
            },
            Some(m) => self.met_for(&m.content)
        }
    }
}


#[test]
fn test_condition_met() {
    let s = "Test".to_string();
    let test_data = vec![
        (MorphemeContent::Invalid, VertexCondition::Nil, true),
        (MorphemeContent::Number(123), VertexCondition::Nil, true),
        (MorphemeContent::Symbol(SymbolType::Add), VertexCondition::Nil, true),
        (MorphemeContent::Identifier(s.clone()), VertexCondition::Nil, true),

        (MorphemeContent::Invalid, VertexCondition::Subgraph(GraphID::BLOCK), true),
        (MorphemeContent::Number(123), VertexCondition::Subgraph(GraphID::BLOCK), true),
        (MorphemeContent::Symbol(SymbolType::Add), VertexCondition::Subgraph(GraphID::BLOCK), true),
        (MorphemeContent::Identifier(s.clone()), VertexCondition::Subgraph(GraphID::BLOCK), true),

        (MorphemeContent::Invalid, VertexCondition::IsSymbol(SymbolType::Add), false),
        (MorphemeContent::Number(123), VertexCondition::IsSymbol(SymbolType::Add), false),
        (MorphemeContent::Symbol(SymbolType::Add), VertexCondition::IsSymbol(SymbolType::Add), true),
        (MorphemeContent::Symbol(SymbolType::Subtract), VertexCondition::IsSymbol(SymbolType::Add), false),
        (MorphemeContent::Identifier(s.clone()), VertexCondition::IsSymbol(SymbolType::Add), false),

        
        (MorphemeContent::Invalid, VertexCondition::IsNumber, false),
        (MorphemeContent::Number(123), VertexCondition::IsNumber, true),
        (MorphemeContent::Symbol(SymbolType::Add), VertexCondition::IsNumber, false),
        (MorphemeContent::Identifier(s.clone()), VertexCondition::IsNumber, false),

        (MorphemeContent::Invalid, VertexCondition::IsIdentifier, false),
        (MorphemeContent::Number(123), VertexCondition::IsIdentifier, false),
        (MorphemeContent::Symbol(SymbolType::Add), VertexCondition::IsIdentifier, false),
        (MorphemeContent::Identifier(s.clone()), VertexCondition::IsIdentifier, true)
    ];

    for t in test_data {
        assert_eq!(t.2, t.1.met_for(&t.0))
    }
}
