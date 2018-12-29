trait Context { 
    type Data;

    fn child<T>(&self) -> Option<Context<Data=T>> {

    }
}

pub struct Context<T> {
    data: T,
    child: Option<Ctxt>,
}

impl Context for Context<T> {
    type Data = T;
}

impl Context<T> {
    fn new(data: T) -> Context<T> {
        Context { data, child: None }
    }

    fn put<U>(self, data: U) -> Context<U> {
        Context { data, child: Some(self) }
    }

    fn get<U>(&self) -> Option<U> {
        if &T == &U {
            Some(&self.data as U)
        } else {
            let child = &self.child.unwrap()
        }
    }
}

