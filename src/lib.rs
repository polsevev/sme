pub fn ParseMD() -> String{
    return "Test".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = ParseMD();
        assert_eq!(result, "Test");
    }
}
