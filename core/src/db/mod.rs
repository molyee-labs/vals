use crate::result::*;

pub trait Type {}

trait WithArgs {
    fn put<T: Into<Type> + ?Sized>(&self, arg: &T) -> Self;
    fn pop<T: Into<Type> + ?Sized>(&self) -> T;
}

pub trait Query: WithArgs {
    fn str(&self) -> &str;
    fn prepare(&self) -> Self;
}

pub trait Function: WithArgs {
    fn name(&self) -> &str;
}

pub trait Database {
    fn get<T: From<Type>>(&self, q: &Query) -> Result<T>;
    fn call<T: From<Type>>(&self, q: &Function) -> Result<T>;
}

macro_rules! func {
    ($name: ident, $($arg: tt)*) => {
        {
            let mut f = Function { name: $name };
            $(f.put($arg);)*
            f
        }
    };
}

macro_rules! query {
    ($query: ident, $($arg: tt)*) => {
        {
            let mut q = Query { str: $query };
            $(q.put($arg);)*
            q
        }
    };
}
