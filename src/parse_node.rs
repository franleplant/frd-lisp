#[derive(Debug, Clone)]
pub struct Node {
    pub ntype: String,
    pub symbol_type: SymbolType,
    pub children: Children,
}

pub type Children = Vec<Node>;

#[derive(Debug, Clone)]
pub enum SymbolType {
    None,
    Symbol(String),
    Num(f64),
    Delimiter(String),
}

impl SymbolType {
    pub fn unwrap_number_ref(&self) -> f64 {
        match self {
            SymbolType::Num(num) => num.clone(),
            _ => panic!("BBBB"),
        }
    }
}

impl Node {
    pub fn new_tree(ntype: String, children: Children) -> Node {
        return Node {
            ntype,
            children,
            symbol_type: SymbolType::None,
        };
    }

    pub fn new_leaf(symbol_type: SymbolType) -> Node {
        return Node {
            ntype: "Leaf".to_string(),
            children: vec![],
            symbol_type,
        };
    }
}
