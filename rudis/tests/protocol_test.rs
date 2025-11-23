#[cfg(test)]
mod tests {
    use rudis::RespValue;

    #[test]
    fn test_parse_array() {
        let input = "*3\r\n$3\r\nSET\r\n$4\r\nname\r\n$5\r\nvalue\r\n";
        let result = RespValue::parse(input).unwrap();

        match result {
            RespValue::Array(ref elements) => {
                assert_eq!(elements.len(), 3);
            }
            _ => panic!("Array가 아닙니다"),
        }

        let cmd = result.to_command_string().unwrap();
        assert_eq!(cmd, "SET name value");
    }

    #[test]
    fn test_parse_bulk_string() {
        let input = "$5\r\nhello\r\n";
        let result = RespValue::parse(input).unwrap();

        match result {
            RespValue::BulkString(s) => assert_eq!(s, "hello"),
            _ => panic!("BulkString이 아닙니다"),
        }
    }

    #[test]
    fn test_parse_null() {
        let input = "$-1\r\n";
        let result = RespValue::parse(input).unwrap();

        assert!(matches!(result, RespValue::Null));
    }

    #[test]
    fn test_complex_command() {
        let input = "*2\r\n$3\r\nGET\r\n$4\r\nname\r\n";
        let result = RespValue::parse(input).unwrap();
        let cmd = result.to_command_string().unwrap();
        assert_eq!(cmd, "GET name");
    }
}