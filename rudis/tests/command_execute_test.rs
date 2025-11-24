#[cfg(test)]
mod tests {
    use rudis::{Command, Store};

    #[test]
    fn test_execute_set_get() {
        let store = Store::new();

        let set_cmd = Command::Set {
            key: "test".to_string(),
            value: "hello".to_string(),
        };
        let result = set_cmd.execute(&store);
        assert_eq!(result, "OK");

        let get_cmd = Command::Get {
            key: "test".to_string(),
        };
        let result = get_cmd.execute(&store);
        assert_eq!(result, "hello");
    }
}
