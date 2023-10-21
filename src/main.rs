use std::fmt;
use std::rc::Rc;

const for_all: &str = "\u{2200}";
const exists: &str = "\u{2203}";
const not: &str = "\u{00ac}";
const and: &str = "\u{22c0}";
const or: &str = "\u{22c1}";
const implies: &str = "\u{2192}";
const equivalent: &str = "\u{2194}";

enum Term {
    Variable(Rc<Variable>),
    Function(Rc<Function>),
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
    Predicate(Rc<Predicate>),
    Equal(Rc<Term>, Rc<Term>), // Only if equality is part of the logic
    Not(Rc<Formula>),
    BinaryConnective(Rc<BinaryConnective>),
}

impl Formula {
    fn from_predicate(pred: Rc<Predicate>) -> Self {
        Self::Predicate(pred)
    }
    fn equality(term1: Rc<Term>, term2: Rc<Term>) -> Self {
        Self::Equal(term1, term2)
    }
    fn negate(formula: Rc<Formula>) -> Self {
        Self::Not(formula)
    }
    fn from_binary_connective(binary_connective: Rc<BinaryConnective>) -> Self {
        Self::BinaryConnective(binary_connective)
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Predicate(pred) => write!(f, "{pred}"),
            Self::Equal(term1, term2) => write!(f, "{term1} = {term2}"),
            Self::Not(func) => write!(f, "{not}{func}"),
            Self::BinaryConnective(bin) => write!(f, "bin"),
        }
    }
}

/// Relation among variables which can be true or false
struct Predicate {
    name: String,
    args: Vec<Rc<Term>>,
}

impl Predicate {
    fn new(name: String, args: Vec<Rc<Term>>) -> Self {
        Self { name, args }
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        for n in 0..self.args.len() - 1 {
            let arg = &self.args[n];
            write!(f, "{arg},")?;
        }
        if let Some(last) = self.args.last() {
            write!(f, "{last})")
        } else {
            write!(f, ")")
        }
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
    args: Vec<Rc<Term>>,
}

enum Quantifier {
    Universal(Rc<Variable>, Rc<Formula>),
    Existential(Rc<Variable>, Rc<Formula>),
}

enum BinaryConnective {
    Conjunction(Rc<Formula>, Rc<Formula>),
    Disjunction(Rc<Formula>, Rc<Formula>),
    Implication(Rc<Formula>, Rc<Formula>),
    Biconditional(Rc<Formula>, Rc<Formula>),
}

impl BinaryConnective {
    fn and(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Conjunction(form1, form2)
    }
    fn or(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Disjunction(form1, form2)
    }
    fn implies(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Implication(form1, form2)
    }
    fn iff(form1: Rc<Formula>, form2: Rc<Formula>) -> Self {
        Self::Biconditional(form1, form2)
    }
}

impl fmt::Display for BinaryConnective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conjunction(form1, form2) => write!(f, "{form1} {and} {form2}"),
            Self::Disjunction(form1, form2) => write!(f, "{form1} {or} {form2}"),
            Self::Implication(form1, form2) => write!(f, "{form1} implies {form2}"),
            Self::Biconditional(form1, form2) => write!(f, "{form1} iff {form2}"),
        }
    }
}

fn main() {
    println!("{for_all},{exists},{not},{and},{or},{implies},{equivalent}");

    let x = Rc::new(Term::Variable(Rc::new(Variable::new("x".to_string()))));
    let y = Rc::new(Term::Variable(Rc::new(Variable::new("y".to_string()))));
    let p = Rc::new(Formula::Predicate(Rc::new(Predicate::new(
        "P".to_string(),
        vec![x.clone(), y.clone()],
    ))));

    let q = Rc::new(Formula::Predicate(Rc::new(Predicate::new(
        "Q".to_string(),
        vec![x, y],
    ))));

    let p_and_q = BinaryConnective::and(p.clone(), q.clone());
    let p_or_q = BinaryConnective::or(p, q);

    println!("{p_and_q} {p_or_q}");
}
