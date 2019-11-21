use super::syntax_graph::{ Graph, GraphID, Node, Vertex };

use super::syntax_graph::VertexCondition as VC;
use super::syntax_graph::VertexTarget as VT;
use super::super::lexer::SymbolType as Sym;

use std::collections::HashMap;

macro_rules! vertex {
    ($condition:expr, $target:expr) => {
        Vertex { condition: $condition, target: $target }
    }
}

pub fn newPL0SyntaxMap() -> HashMap<GraphID, Graph> {
    newPL0Syntax().into_iter().map(|x| (x.id.clone(), x)).collect()
}

pub fn newPL0Syntax() -> Vec<Graph> {
  vec! [
    Graph {
      id: "PROGRAM".to_string(),
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph("BLOCK".to_string()), VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Point), VT::EndOfGraph)]
        }
      ]
    },
    Graph {
      id: "BLOCK".to_string(),
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Const), VT::Node(1)),
            vertex!(VC::Nil, VT::Node(2) )
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("CONST".to_string()), VT::Node(2))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Var), VT::Node(3) ),
            vertex!(VC::Nil, VT::Node(4) )
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("VAR".to_string()), VT::Node(4))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Procedure), VT::Node(5) ),
            vertex!(VC::Nil, VT::Node(6) )
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("PROCEDURE".to_string()), VT::Node(6))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("STATEMENT".to_string()), VT::EndOfGraph)]
        }
      ]
    },
    Graph {
      id: "CONST".to_string(),
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Equals), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::IsNumber, VT::Node(3))]
        },
        Node {
          vertices: vec![
              vertex!(VC::IsSymbol(Sym::Comma), VT::Node(0)),
              vertex!(VC::IsSymbol(Sym::Semicolon), VT::EndOfGraph )
          ]
        }
      ]
    },
    Graph {
      id: "VAR".to_string(),
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::Node(1))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Comma), VT::Node(0)),
            vertex!(VC::IsSymbol(Sym::Semicolon), VT::EndOfGraph)
          ]
        }
      ]
    },
    Graph {
      id: "PROCEDURE".to_string(),
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Semicolon), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("BLOCK".to_string()), VT::Node(3))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Semicolon), VT::EndOfGraph)]
        }
      ]
    },
    Graph {
      id: "STATEMENT".to_string(),
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsIdentifier, VT::Node(1)),
            vertex!(VC::IsSymbol(Sym::If), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::While), VT::Node(6)),
            vertex!(VC::IsSymbol(Sym::Begin), VT::Node(9)),
            vertex!(VC::IsSymbol(Sym::Call), VT::Node(11)),
            vertex!(VC::IsSymbol(Sym::QuestionMark), VT::Node(12)),
            vertex!(VC::IsSymbol(Sym::ExclamationMark), VT::Node(13)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        },

        // Assignment
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Assignment), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("EXPRESSION".to_string()), VT::EndOfGraph)]
        },

        // IF
        Node {
          vertices: vec![vertex!(VC::Subgraph("CONDITION".to_string()), VT::Node(4))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Then), VT::Node(5))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("STATEMENT".to_string()), VT::EndOfGraph)]
        },

        // WHILE
        Node {
          vertices: vec![vertex!(VC::Subgraph("CONDITION".to_string()), VT::Node(7))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Then), VT::Node(8))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("STATEMENT".to_string()), VT::EndOfGraph)]
        },

        // BEGIN / END
        Node {
          vertices: vec![vertex!(VC::Subgraph("STATEMENT".to_string()), VT::Node(10))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Semicolon), VT::Node(9)),
            vertex!(VC::IsSymbol(Sym::End), VT::EndOfGraph)
          ]
        },

        // CALL
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::EndOfGraph)]
        },

        // ?
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::EndOfGraph)]
        },

        // !
        Node {
          vertices: vec![vertex!(VC::Subgraph("EXPRESSION".to_string()), VT::EndOfGraph)]
        }
      ]
    },
    Graph {
      id: "CONDITION".to_string(),
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Odd), VT::Node(1)),
            vertex!(VC::Subgraph("EXPRESSION".to_string()), VT::Node(2))
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("EXPRESSION".to_string()), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Equals), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::Hash), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::Lesser), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::Greater), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::LesserOrEqual), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::GreaterOrEqual), VT::Node(3))
          ]
        },
        Node {
          vertices: vec![
            vertex!(VC::Subgraph("EXPRESSION".to_string()), VT::EndOfGraph)
          ]
        }
      ]
    },
    Graph {
      id: "EXPRESSION".to_string(),
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Subtract), VT::Node(1)),
            vertex!(VC::Nil, VT::Node(1))
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("TERM".to_string()), VT::Node(2))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Add), VT::Node(1)),
            vertex!(VC::IsSymbol(Sym::Subtract), VT::Node(1)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        }
      ]
    },
    Graph {
      id: "TERM".to_string(),
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph("FACTOR".to_string()), VT::Node(1))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Multiply), VT::Node(2)),
            vertex!(VC::IsSymbol(Sym::Divide), VT::Node(2)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        }
      ]
    },
    Graph {
      id: "FACTOR".to_string(),
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsNumber, VT::EndOfGraph),
            vertex!(VC::IsSymbol(Sym::RoundOpeningBrace), VT::Node(1)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph("EXPRESSION".to_string()), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::RoundClosingBrace), VT::EndOfGraph)]
        }
      ]
    }
  ]
}
