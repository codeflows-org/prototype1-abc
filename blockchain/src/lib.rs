pub struct Blockchain {}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
