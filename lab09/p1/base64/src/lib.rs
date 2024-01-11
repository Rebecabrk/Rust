pub fn encode(input: &mut [u8]) -> String {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = base64("Beca");
        assert_eq!(result, "QmVjYQ==");
    }
}
