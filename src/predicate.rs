use crate::variable::Variable;
use std::rc::Rc;
use std::fmt;

pub struct Predicate {
    name: String,
    arguments: Vec<Rc<Variable>>,
}

impl Predicate {
    pub fn new(name: String, arguments: Vec<Rc<Variable>>) -> Self {
	Self {name, arguments}
    }
}

impl fmt::Display for Predicate {
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

macro_rules! pred {
    ($name:expr, $($args:expr),*) => {
        Rc::new(Predicate::new(format!($name), vec![$($args.clone()),*]))
    };
}

pub(crate) use pred;

