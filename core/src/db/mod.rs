use crate::result::*;
use std::sync::Mutex;

pub trait Type {}

trait WithArgs {
    fn put<T: Into<Type>>(&self, arg: T) -> Self;
    fn pop(&self) -> impl Into<Type>;
}

#[derive(WithArgs)]
pub trait Query {
    fn str(&self) -> &str;
    fn prepare(&self) -> Self;
}

#[derive(WithArgs)]
pub trait Function {
    fn name(&self) -> &str;
}

pub trait Database {
    fn get<T: From<Type>>(&self, q: &Query) -> Result<T>;
    fn call<T: From<Type>>(&self, q: &Func) -> Result<T>;
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
