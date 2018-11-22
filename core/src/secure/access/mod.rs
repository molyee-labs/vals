use ::result::*;

pub fn is_allowed(c:Context) -> Result<bool> {
    Ok(true)
}

pub fn is_denied(c:Context) -> Result<bool> {
    is_allowed(c).map(|b| !b)
}