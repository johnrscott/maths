use std::fmt;

enum Term {
    Variable(Variable),
    Function(Function),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	match self {
	    Self::Variable(x) => write!(f, "{x}"),
	    Self::Function(f) => unimplemented!(),
	}
    }
}


/// Primitive item that will be reasoned about
struct Variable {
    name: String,
}

impl Variable {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


macro_rules! var {
    ($name:expr) => {
        Term::Variable(Variable::new(format!($name)))
    };
}

/// Well-formed formula
enum Formula {
    Predicate(Predicate),
    Equal(Term, Term), // Only if equality is part of the logic
    Not(Box<Formula>),
    BinaryConnective(Box<Formula>, Box<Formula>),
}

/// Relation among variables which can be true or false
struct Predicate {
    name: String,
    args: Vec<Term>,
}

impl Predicate {
    fn new(name: String, args: Vec<Term>) -> Self {
        Self { name, args }
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
	for arg in &self.args {
	    write!(f, "{arg},")?;
	}
	write!(f, ")")
    }
}

macro_rules! pred {
    ($name:expr, $($args:expr),*) => {
        Predicate::new(format!($name), vec![$($args),*])
    };
}

/// Rule for obtaining a variable from other variables
struct Function {
    name: String,
    args: Vec<Term>,
}

enum Quantifier {
    Universal(Variable, Formula),
    Existential(Variable, Formula),
}

enum BinaryConnective {
    Conjunction(Formula, Formula),
    Disjunction(Formula, Formula),
    Implication(Formula, Formula),
    Biconditional(Formula, Formula),
}

fn main() {
    let for_all = "\u{2200}";
    let exists = "\u{2203}";
    let not = "\u{00ac}";
    let and = "\u{22c0}";
    let or = "\u{22c1}";
    let implies = "\u{2192}";
    let equivalent = "\u{2194}";

    println!("{for_all},{exists},{not},{and},{or},{implies},{equivalent}");

    let x = var!("x");
    let y = var!("y");
    let p = pred!("P", x, y);
    println!("{p}");
}
