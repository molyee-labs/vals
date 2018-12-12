use crate::result::Result;

pub trait Storage {
    fn get<I, T>(id: I) -> Result<T>;
    fn retrieve<I, T>(id: I) -> Result<T>;
    fn put<I, T>(val: T) -> Result<I>;
    fn set<I, T>(id: I, val: T) -> Result<()>;
    fn drop<I, T>(id: I) -> Result<T>;
}
