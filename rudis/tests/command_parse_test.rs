#[cfg(test)]
mod tests {
    use rudis::Command;

    #[test]
    fn test_parse_set() {
        let cmd = Command::parse("SET name rudis").unwrap();
        assert_eq!(
            cmd,
            Command::Set {
                key: "name".to_string(),
                value: "rudis".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_get() {
        let cmd = Command::parse("GET name").unwrap();
        assert_eq!(
            cmd,
            Command::Get {
                key: "name".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_del() {
        let cmd = Command::parse("DEL name").unwrap();
        assert_eq!(
            cmd,
            Command::Del {
                key: "name".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_expire() {
        let cmd = Command::parse("EXPIRE key 10").unwrap();
        assert_eq!(
            cmd,
            Command::Expire {
                key: "key".to_string(),
                seconds: 10,
            }
        );
    }

    #[test]
    fn test_parse_ttl() {
        let cmd = Command::parse("TTL key").unwrap();
        assert_eq!(
            cmd,
            Command::Ttl {
                key: "key".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_exit() {
        let cmd = Command::parse("EXIT").unwrap();
        assert_eq!(cmd, Command::Exit);
    }

    #[test]
    fn test_parse_unknown() {
        // 형식에 맞지 않는 명령어를 입력시
        let result = Command::parse("EXPIRE key abc");
        assert!(result.is_err());
    }
}
