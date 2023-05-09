use std::{collections::{VecDeque, HashMap}, env};

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() > 1 {
        println!("AST: {}", parse(args[1].clone()).unwrap());
        println!("EVAL: {}", eval(parse(args[1].clone()).unwrap(), new_context()));
    }
}

fn new_context() -> Context {
    let mut c = Context::new();
    c.insert("+".to_string(), Exp::Proc(|args| {
        return match (&args[0], &args[1]) {
            (Exp::Atom(Atom::Number(x)), Exp::Atom(Atom::Number(y))) => Exp::Atom(Atom::Number(x+y)),
            _ => panic!(""),
        }
    }));
    
    c.insert("*".to_string(), Exp::Proc(|args| {
        return match (&args[0], &args[1]) {
            (Exp::Atom(Atom::Number(x)), Exp::Atom(Atom::Number(y))) => Exp::Atom(Atom::Number(x*y)),
            _ => panic!(""),
        }
    }));
    
    c.insert("/".to_string(), Exp::Proc(|args| {
        return match (&args[0], &args[1]) {
            (Exp::Atom(Atom::Number(x)), Exp::Atom(Atom::Number(y))) => Exp::Atom(Atom::Number(x/y)),
            _ => panic!(""),
        }
    }));

    c
}

fn tokenize(input: String) -> VecDeque<String> {
    input.replace("(", "( ").replace(")", " )").split(" ").map(|s| s.to_string()).collect()
}

type Context = HashMap<String, Exp>;

#[derive(Clone)]
pub enum Exp {
    Atom(Atom),
    List(List),
    Proc(fn(args: &[Exp]) -> Exp),
}

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
            _ => {
                repr.push_str(format!("proc").as_str());
            }
        }
        write!(f, "{}", repr)
    }
}

#[derive(Clone)]
pub struct List(Vec<Exp>);

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let content = self.0.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ");
        write!(f, "({})", content)
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Number(f64),
    Symbol(String),
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut repr = String::new();
        match &self {
            Self::Number(n) => {
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

fn to_atom(token: String) -> Atom {
    match token.parse::<f64>() {
        Ok(f) => return Atom::Number(f),
        _ => return Atom::Symbol(token)
    }
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
            return Ok(Exp::Atom(to_atom(token)));
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
 

fn eval(exp: Exp, ctx: Context) -> Exp {
    match exp {
        Exp::Atom(Atom::Symbol(s)) => return ctx.get(&s).unwrap().clone(),
        Exp::Atom(n) => return Exp::Atom(n),
        _ => {
            if let Exp::List(l) = exp {
                let proc = eval(l.0[0].clone(), ctx.clone());
                let args: Vec<Exp> = l.0[1..].iter().map(|a| eval(a.clone(), ctx.clone())).collect();

                if let Exp::Proc(p) = proc {
                    return p(&args[..]);
                }
            }

            panic!();
        }
    }
}
