use lexer::Token;
use parse_node::{Node, SymbolType};
use std::collections::HashMap;

pub type Prods = HashMap<&'static str, Vec<Vec<&'static str>>>;

lazy_static! {
    pub static ref PRODS: Prods = {
        let mut map = HashMap::new();

        map.insert(
            "Program",
            vec![vec!["Expression"], vec!["Expression", "Program"]],
        );

        map.insert(
            "Expression",
            vec![vec!["(", ")"], vec!["(", "List", ")"], vec!["Atom"]],
        );

        map.insert("List", vec![vec!["Expression", "List"], vec!["Expression"]]);

        map.insert("Atom", vec![vec!["Id"], vec!["Num"], vec!["PrimitiveOp"]]);

        return map;
    };
}

/// A hook to run some trasnformations over a parse tree, useful
/// to simplify the parse tree and avoid extra nodes
pub fn postprocess_tree(tree: Node, prod: (&str, &Vec<&str>)) -> Node {
    let (non_terminal, right_side) = prod;
    //TODO make this a match

    // TODO do the same for Expression Program
    if non_terminal == "List" && right_side == &vec!["Expression", "List"] {
        return flatten_list(tree);
    }

    if non_terminal == "Expression" && right_side == &vec!["(", "List", ")"] {
        return remove_parenthesis(tree);
    }

    return tree;
}

/// A hook to create the appropriate leaf_symbol from a given token.
/// Use it to trasnform numbers and such.
pub fn new_leaf_symbol(token: &Token) -> Node {
    let lexeme = token.lexeme.to_string();
    let kind = token.kind.as_ref();

    let leaf_symbol = match kind {
        "(" | ")" => SymbolType::Delimiter(lexeme),
        "Id" | "PrimitiveOp" => SymbolType::Symbol(lexeme),
        "Num" => {
            let num = lexeme.parse::<f64>().expect("Badly formated number");
            SymbolType::Num(num)
        }
        _ => panic!("Unexpected token {:?}", token),
    };

    let leaf = Node::new_leaf(leaf_symbol);

    return leaf;
}

pub fn flatten_list(node: Node) -> Node {
    let flat_children = node
        .children
        .into_iter()
        .enumerate()
        .flat_map(|(i, node)| {
            if i == 1 && node.ntype == "List" {
                return node.children;
            }
            return vec![node];
        }).collect();

    return Node::new_tree(node.ntype, flat_children);
}

pub fn remove_parenthesis(mut node: Node) -> Node {
    println!("{:#?}", node);

    // TODO asserts
    let leaf = node.children.remove(1);
    node.children = vec![leaf];
    return node;
}
