enum Term {
    Variable(Variable),
    Function(Function),
}

/// Primitive item that will be reasoned about
struct Variable {
    name: String,
}

/// Well-formed formula
enum Formula {
    Predicate(Predicate),
    Equal(Term, Term), // Only if equality is part of the logic
    Not(Box<Formula>),
    BinaryConnective()
}

/// Relation among variables which can be true or false
struct Predicate {
    arity: usize,
    name: String,
    binding: Vec<Term>,
}

/// Rule for obtaining a variable from other variables
struct Function {
    arity: usize,
    name: String,
    binding: Vec<Term>
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
}
