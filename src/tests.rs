#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

    use crate::format;

    #[test]
    fn default_space_padding() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%20%20", datetime);
        assert_eq!(result, "                 %20                 %20".to_string());
    }

    #[test]
    fn zero_padding() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%020%020", datetime);
        assert_eq!(result, "0000000000000000%0200000000000000000%020".to_string());
    }

    #[test]
    fn no_padding() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%-20%-20", datetime);
        assert_eq!(result, "%-20%-20".to_string());
    }

    #[test]
    fn space_padding() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%_20%_20", datetime);
        assert_eq!(result, "                %_20                %_20".to_string());
    }

    #[test]
    fn mixed_padding() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%20%-20", datetime);
        assert_eq!(result, "                 %20%-20".to_string());
    }

    #[test]
    fn custom_char_padding() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%*20", datetime);
        assert_eq!(result, "%*20".to_string());
    }

    #[test]
    fn gnu_date_test_case_1() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%Y-%m-%d", datetime);
        let expected = datetime.format("%Y-%m-%d").to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn gnu_date_test_case_2() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%H:%M:%S", datetime);
        let expected = datetime.format("%H:%M:%S").to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn gnu_date_test_case_3() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%A, %B %d, %Y", datetime);
        let expected = datetime.format("%A, %B %d, %Y").to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn gnu_date_test_case_4() {
        let datetime = Local::now().with_timezone(Local::now().offset());
        let result = format("%a %b %e %H:%M:%S %Z %Y", datetime);
        let expected = datetime.format("%a %b %e %H:%M:%S %Z %Y").to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn gnu_date_test_case_5() {
        let datetime = Local.with_ymd_and_hms( 2024, 12, 14, 0, 0, 0).single().unwrap().with_timezone(Local::now().offset());
        let result = format("%_20F", datetime);
        let expected = "          2024-12-14".to_string();
        assert_eq!(result, expected);
    }
    #[test]
    fn gnu_date_test_case_6() {
        let datetime = Local.with_ymd_and_hms( 2024, 12, 14, 0, 0, 0).single().unwrap().with_timezone(Local::now().offset());
        let result = format("%_20D", datetime);
        let expected = "            12/14/24".to_string();
        assert_eq!(result, expected);
    }
}
