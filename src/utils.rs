pub fn flatten<T, E>(res: Result<Result<T, E>, E>) -> Result<T, E> {
    match res {
        Ok(res) => res,
        Err(err) => Err(err),
    }
}
