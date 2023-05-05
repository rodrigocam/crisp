use std::{collections::{VecDeque, HashMap}, env};


fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() > 1 {
        println!("AST: {}", parse(args[1].clone()).unwrap());
    }
}

fn tokenize(input: String) -> VecDeque<String> {
    input.replace("(", "( ").replace(")", " )").split(" ").map(|s| s.to_string()).collect()
}

#[derive(Debug)]
pub enum Exp {
    Atom(Atom),
    List(List)
}

#[derive(Debug)]
pub struct List(Vec<Exp>);

impl std::fmt::Display for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut repr = String::new();
        match &self {
            Self::Atom(a) => {
                repr.push_str(format!("{}", a).as_str());
            },
            Self::List(e) => {
                repr.push_str(format!("{}", e).as_str());
            }
        }
        write!(f, "{}", repr)
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let content = self.0.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ");
        write!(f, "({})", content)
    }
}

#[derive(Debug)]
pub enum Atom {
    Int(i64),
    Float(f64),
    Symbol(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut repr = String::new();
        match &self {
            Self::Int(n) => {
                repr.push_str(format!("{:?}", n).as_str());
            },
            Self::Float(n) => {
                repr.push_str(format!("{:?}", n).as_str());
            },
            Self::Symbol(s) => {
                repr.push_str(format!("{:?}", s).as_str());
            }
        }
        write!(f, "{}", repr)
    }
}

#[derive(Clone, Debug)]
pub enum ParsingError {
    SyntaxError,
    UnexpectedEOF
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParsingError")
    }
}

fn atom(t: String) -> Atom {
    let int = t.parse::<i64>();
    let float = t.parse::<i64>();

    if int.is_ok() {
        return Atom::Int(int.unwrap() as i64);
    } else if float.is_ok() {
        return Atom::Float(float.unwrap() as f64);
    }

    return Atom::Symbol(t);
}

fn read_from_tokens(tokens: &mut VecDeque<String>) -> Result<Exp, ParsingError> {
    if tokens.len() == 0 {
        return Err(ParsingError::UnexpectedEOF);
    }
     
    let token = tokens.pop_front().unwrap();

    match token.as_str() {
        "(" => {
            let mut list = List(Vec::new());
            while tokens[0] != ")" {
               list.0.push(read_from_tokens(tokens)?);
            }
            tokens.pop_front();
            return Ok(Exp::List(list));
        },
        ")" => {
            return Err(ParsingError::UnexpectedEOF);
        },
        _ => {
            return Ok(Exp::Atom(atom(token)));
        }
    }
}

/// Example
///```
///let s = "(add 1 2)";
///assert_eq!(parse(s.replace("(", "( ").replace(")", " )").collect()), (Symbol::from("add"),
///(Arg::Number(1), Arg::Number(2)))
///```

fn parse(input: String) -> Result<Exp, ParsingError> {
    let mut tokens = tokenize(input);
    read_from_tokens(&mut tokens)
}
 

// fn eval(expr: Expr, ctx: Context)
