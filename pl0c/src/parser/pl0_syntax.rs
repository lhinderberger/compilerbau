use super::{ Graph, GraphID, Node, Vertex };

use super::VertexCondition as VC;
use super::VertexTarget as VT;
use super::super::lexer::SymbolType as Sym;

use std::collections::HashMap;

macro_rules! vertex {
    ($condition:expr, $target:expr) => {
        Vertex { condition: $condition, target: $target }
    }
}

lazy_static! {
  pub static ref PL0_SYNTAX: HashMap<GraphID, Graph> = {
    let mut syntax = HashMap::new();

    syntax.insert(GraphID::PROGRAM, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::BLOCK), VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Point), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::BLOCK, Graph {
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Const), VT::Node(1)),
            vertex!(VC::Nil, VT::Node(2) )
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::CONST), VT::Node(2))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Var), VT::Node(3) ),
            vertex!(VC::Nil, VT::Node(4) )
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::VAR), VT::Node(4))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Procedure), VT::Node(5) ),
            vertex!(VC::Nil, VT::Node(6) )
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::PROCEDURE), VT::Node(6))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::STATEMENT), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::CONST, Graph {
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
    });
    syntax.insert(GraphID::VAR, Graph {
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
    });
    syntax.insert(GraphID::PROCEDURE, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Semicolon), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::BLOCK), VT::Node(3))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Semicolon), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::STATEMENT, Graph {
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsIdentifier, VT::Node(1)),
            vertex!(VC::IsSymbol(Sym::If), VT::Node(2)),
            vertex!(VC::IsSymbol(Sym::While), VT::Node(3)),
            vertex!(VC::IsSymbol(Sym::Begin), VT::Node(4)),
            vertex!(VC::IsSymbol(Sym::Call), VT::Node(5)),
            vertex!(VC::IsSymbol(Sym::QuestionMark), VT::Node(6)),
            vertex!(VC::IsSymbol(Sym::ExclamationMark), VT::Node(7)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        },

        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::ASSIGNMENT), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::IF), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::WHILE), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::COMPOUND), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::CALL), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::INPUT), VT::EndOfGraph)]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::OUTPUT), VT::EndOfGraph)]
        }        
      ]
    });
    syntax.insert(GraphID::ASSIGNMENT, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Assignment), VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::EXPRESSION), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::IF, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::CONDITION), VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Then), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::STATEMENT), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::WHILE, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::CONDITION), VT::Node(1))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::Do), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::STATEMENT), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::COMPOUND, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::STATEMENT), VT::Node(1))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Semicolon), VT::Node(0)),
            vertex!(VC::IsSymbol(Sym::End), VT::EndOfGraph)
          ]
        }
      ]
    });
    syntax.insert(GraphID::CALL, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::INPUT, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::IsIdentifier, VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::OUTPUT, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::EXPRESSION), VT::EndOfGraph)]
        }
      ]
    });
    syntax.insert(GraphID::CONDITION, Graph {
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Odd), VT::Node(1)),
            vertex!(VC::Subgraph(GraphID::EXPRESSION), VT::Node(2))
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::EXPRESSION), VT::EndOfGraph)]
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
            vertex!(VC::Subgraph(GraphID::EXPRESSION), VT::EndOfGraph)
          ]
        }
      ]
    });
    syntax.insert(GraphID::EXPRESSION, Graph {
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Subtract), VT::Node(1)),
            vertex!(VC::Nil, VT::Node(1))
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::TERM), VT::Node(2))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Add), VT::Node(1)),
            vertex!(VC::IsSymbol(Sym::Subtract), VT::Node(1)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        }
      ]
    });
    syntax.insert(GraphID::TERM, Graph {
      nodes: vec![
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::FACTOR), VT::Node(1))]
        },
        Node {
          vertices: vec![
            vertex!(VC::IsSymbol(Sym::Multiply), VT::Node(2)),
            vertex!(VC::IsSymbol(Sym::Divide), VT::Node(2)),
            vertex!(VC::Nil, VT::EndOfGraph)
          ]
        }
      ]
    });
    syntax.insert(GraphID::FACTOR, Graph {
      nodes: vec![
        Node {
          vertices: vec![
            vertex!(VC::IsNumber, VT::EndOfGraph),
            vertex!(VC::IsSymbol(Sym::RoundOpeningBrace), VT::Node(1)),
            vertex!(VC::IsIdentifier, VT::EndOfGraph)
          ]
        },
        Node {
          vertices: vec![vertex!(VC::Subgraph(GraphID::EXPRESSION), VT::Node(2))]
        },
        Node {
          vertices: vec![vertex!(VC::IsSymbol(Sym::RoundClosingBrace), VT::EndOfGraph)]
        }
      ]
    });

    syntax
  };
}
