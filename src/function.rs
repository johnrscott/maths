use crate::term::Term;
use std::rc::Rc;
use std::fmt;

/// Everything in this file is very similar to Predicate

/// Rule for obtaining a variable from other variables
pub struct Function {
    name: String,
    arguments: Vec<Rc<Term>>,
}

impl Function {
    pub fn new(name: String, arguments: Vec<Rc<Term>>) -> Self {
	Self {name, arguments}
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(", self.name)?;
        for n in 0..self.arguments.len() - 1 {
            let arg = &self.arguments[n];
            write!(f, "{arg},")?;
        }
        if let Some(last) = self.arguments.last() {
            write!(f, "{last})")
        } else {
            write!(f, ")")
        }
    }
}

macro_rules! func {
    ($name:expr, $($args:expr),*) => {
        Rc::new(Function::new(format!($name), vec![$($args.clone()),*]))
    };
}

pub(crate) use func;




