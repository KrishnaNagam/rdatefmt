#[cfg(test)]
mod tests {
    use crate::format;
    #[test]
    fn default_space_padding() {
        let result = format("%20%20");
        assert_eq!(result,"                 %20                 %20".to_string());
    }

    #[test]
    fn zero_padding() {
        let result = format("%020%020");
        assert_eq!(result,"0000000000000000%0200000000000000000%020".to_string());
    }

    #[test]
    fn no_padding() {
        let result = format("%-20%-20");
        assert_eq!(result,"%-20%-20".to_string());
    }

    #[test]
    fn space_padding() {
        let result = format("%_20%_20");
        assert_eq!(result,"                %_20                %_20".to_string());
    }
}
