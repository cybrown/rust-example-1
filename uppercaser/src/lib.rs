// A dummy example stateless service

#[derive(Copy, Clone)]
pub struct Uppercaser {}

impl Uppercaser {
    pub fn to_uppercase(&self, str: String) -> String {
        str.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uppercase_a() {
        let a = Uppercaser {};
        assert_eq!(a.to_uppercase("a".to_owned()), "A");
    }
}
