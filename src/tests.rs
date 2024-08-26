#[cfg(test)]
mod tests {
    use crate::format;
    #[test]
    fn it_works() {
        let result = format("%20%20");
        assert_eq!(result,"                 %20                 %20".to_string());
    }
}