#[derive(Debug, Clone)]
pub struct Node {
    pub ntype: String,
    pub stype: SymbolType,
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

impl Node {
    pub fn new_tree(ntype: String, children: Children) -> Node {
        return Node {
            ntype,
            children,
            stype: SymbolType::None,
        };
    }

    pub fn new_leaf(stype: SymbolType) -> Node {
        return Node {
            ntype: "Leaf".to_string(),
            children: vec![],
            stype,
        };
    }
}
