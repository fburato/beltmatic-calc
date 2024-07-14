use clap::Parser;
use std::{cell::RefCell, collections::HashMap, fmt::Display, process::exit, rc::Rc};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Operation {
    ADD,
    MULT,
    DIV,
    SUB,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    max_number: i32,
    #[arg(long)]
    max_size: usize,
    #[arg(long)]
    operations: Option<String>,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ADD => write!(f, "+"),
            Self::SUB => write!(f, "-"),
            Self::MULT => write!(f, "*"),
            Self::DIV => write!(f, "/"),
        }
    }
}

trait Node: Display {
    fn eval(&self) -> Option<i32>;
}

struct NumNode {
    value: Rc<RefCell<i32>>,
}

impl Display for NumNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.as_ref().borrow())
    }
}

impl Node for NumNode {
    fn eval(&self) -> Option<i32> {
        Some(*self.value.as_ref().borrow())
    }
}

impl NumNode {
    fn new(value: Rc<RefCell<i32>>) -> NumNode {
        NumNode { value }
    }
}

struct BinaryNode {
    left: Rc<dyn Node>,
    right: Rc<dyn Node>,
    operation: Rc<RefCell<Operation>>,
}

impl Display for BinaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        self.left.as_ref().fmt(f)?;
        write!(f, "{}", self.operation.as_ref().borrow())?;
        self.right.as_ref().fmt(f)?;
        write!(f, ")")
    }
}

impl Node for BinaryNode {
    fn eval(&self) -> Option<i32> {
        match *self.operation.as_ref().borrow() {
            Operation::ADD => self
                .left
                .eval()
                .and_then(|l| self.right.eval().map(|r| l + r)),
            Operation::SUB => self
                .left
                .eval()
                .and_then(|l| self.right.eval().map(|r| l - r)),
            Operation::MULT => self
                .left
                .eval()
                .and_then(|l| self.right.eval().map(|r| l * r)),
            Operation::DIV => self
                .left
                .eval()
                .and_then(|l| self.right.eval().filter(|r| *r != 0).map(|r| l / r)),
        }
    }
}

impl BinaryNode {
    fn new(l: Rc<dyn Node>, r: Rc<dyn Node>, o: Rc<RefCell<Operation>>) -> BinaryNode {
        BinaryNode {
            left: l,
            right: r,
            operation: o,
        }
    }
}

struct Composed {
    ints: Vec<Rc<RefCell<i32>>>,
    ops: Vec<Rc<RefCell<Operation>>>,
    alternatives: Vec<Rc<dyn Node>>,
}

fn make_options(size: usize) -> Composed {
    let mut ints: Vec<Rc<RefCell<i32>>> = Vec::with_capacity(size);
    for _ in 0..size {
        ints.push(Rc::new(RefCell::new(0)));
    }
    let mut num_nodes: Vec<Rc<dyn Node>> = Vec::with_capacity(size);
    for i in 0..size {
        num_nodes.push(Rc::new(NumNode::new(ints[i].clone())));
    }
    let mut operations: Vec<Rc<RefCell<Operation>>> = Vec::with_capacity(size - 1);
    for _ in 0..(size - 1) {
        operations.push(Rc::new(RefCell::new(Operation::ADD)));
    }
    let alternatives = calculate_parenthesisations(0, size, &num_nodes, &operations);
    Composed {
        ints,
        ops: operations,
        alternatives,
    }
}

fn calculate_parenthesisations(
    left: usize,
    right: usize,
    nodes: &Vec<Rc<dyn Node>>,
    operations: &Vec<Rc<RefCell<Operation>>>,
) -> Vec<Rc<dyn Node>> {
    if left + 1 == right {
        return vec![nodes[left].clone()];
    }

    if left + 2 == right {
        return vec![Rc::new(BinaryNode::new(
            nodes[left].clone(),
            nodes[left + 1].clone(),
            operations[left].clone(),
        ))];
    }
    let mut result: Vec<Rc<dyn Node>> = Vec::new();
    for i in (left + 1)..right {
        let left_combinations = calculate_parenthesisations(left, i, nodes, operations);
        let right_combinations = calculate_parenthesisations(i, right, nodes, operations);
        for left_node in left_combinations.iter() {
            for right_node in right_combinations.iter() {
                result.push(Rc::new(BinaryNode::new(
                    left_node.clone(),
                    right_node.clone(),
                    operations[i - 1].clone(),
                )))
            }
        }
    }
    return result;
}

struct OperationDictionary {
    operations: Vec<Operation>,
    indexes: HashMap<Operation, usize>,
}

impl OperationDictionary {
    fn new(options: &Vec<String>) -> Option<OperationDictionary> {
        let mut operations: Vec<Operation> = vec![];
        let mut indexes: HashMap<Operation, usize> = HashMap::new();
        let mut error = false;
        for option in options.iter() {
            match option.as_str() {
                "+" => {
                    operations.push(Operation::ADD);
                    indexes.insert(Operation::ADD, operations.len() - 1);
                }
                "-" => {
                    operations.push(Operation::SUB);
                    indexes.insert(Operation::SUB, operations.len() - 1);
                }
                "*" => {
                    operations.push(Operation::MULT);
                    indexes.insert(Operation::MULT, operations.len() - 1);
                }
                "/" => {
                    operations.push(Operation::DIV);
                    indexes.insert(Operation::DIV, operations.len() - 1);
                }
                _ => {
                    error = true;
                }
            }
        }
        if error {
            None
        } else {
            Some(OperationDictionary {
                operations,
                indexes,
            })
        }
    }

    fn operation(&self, index: usize) -> Operation {
        self.operations[index]
    }

    fn index(&self, operation: &Operation) -> usize {
        *self.indexes.get(operation).unwrap()
    }

    fn max_operation(&self) -> Operation {
        self.operations[self.operations.len() - 1]
    }
}

fn main() {
    let args = Args::parse();
    if args.max_number <= 0 {
        println!("max_number must be > 0, was {}", args.max_number);
        exit(1);
    }
    let operations_arg = args.operations.unwrap_or("+,-,*,/".to_string());
    let operations = operations_arg.split(",").map(|s| s.to_string()).collect();
    let operation_dictionary = OperationDictionary::new(&operations);
    if operation_dictionary.is_none() {
        println!(
            "unrecognised operations found, allowed=[+,-,*,/], provided={:?}",
            operations
        );
        exit(1);
    }
    let operation_dictionary = operation_dictionary.unwrap();
    let maximum_number = args.max_number;
    let maximum_size = args.max_size;
    let mut dictionary: HashMap<i32, (usize, Vec<String>)> = HashMap::new();
    let mut maximum_composed = 1;
    for size in 1..(maximum_size + 1) {
        let composed = make_options(size);
        let mut op_finished = false;
        while !op_finished {
            for i in 0..composed.ints.len() {
                composed.ints[i].replace(1);
            }
            let limit = maximum_number;
            let mut finished = false;
            while !finished {
                for alternative in &composed.alternatives {
                    if let Some(v) = alternative.eval() {
                        if v > maximum_composed {
                            maximum_composed = v;
                        }
                        if !dictionary.contains_key(&v) {
                            dictionary.insert(v, (size, vec![format!("{}", alternative)]));
                        } else {
                            let (max_size, options) = dictionary.get_mut(&v).unwrap();
                            if *max_size == size {
                                options.push(format!("{}", alternative));
                            }
                        }
                    }
                }
                let mut i: usize = 0;
                while i < composed.ints.len() && *composed.ints[i].borrow() == limit {
                    composed.ints[i].replace(1);
                    i += 1;
                }
                if i < composed.ints.len() {
                    let current = *composed.ints[i].borrow();
                    composed.ints[i].replace(current + 1);
                } else {
                    finished = true;
                }
            }

            let mut op: usize = 0;
            while op < composed.ops.len()
                && *composed.ops[op].borrow() == operation_dictionary.max_operation()
            {
                composed.ops[op].replace(operation_dictionary.operation(0));
                op += 1;
            }
            if op < composed.ops.len() {
                let current_op = operation_dictionary.index(&composed.ops[op].borrow());
                composed.ops[op].replace(operation_dictionary.operation(current_op + 1));
            } else {
                op_finished = true;
            }
        }
    }

    for v in 1..(maximum_composed + 1) {
        println!(
            "{} -> {}",
            v,
            dictionary
                .get(&v)
                .map(|(size, options)| format!("({}) {:?}", size, options))
                .unwrap_or("None".to_string())
        );
    }
}
