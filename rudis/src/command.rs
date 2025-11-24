use crate::Store;

#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Expire { key: String, seconds: i64 },
    Ttl { key: String },

    Ping,
    ClientSetName { name: String },
    ClientSetInfo { data: String },
    Hello { version: i64 },
    Info,
    Quit,

    Exit,
    Unknown,
}

impl Command {
    // CLI에서 입력된 문자열을 Command enum으로 파싱하는 함수
    pub fn parse(input: &str) -> Result<Command, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["SET", key, value] => Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }),

            ["GET", key] => Ok(Command::Get {
                key: key.to_string(),
            }),

            ["DEL", key] => Ok(Command::Del {
                key: key.to_string(),
            }),

            ["EXPIRE", key, seconds] => match seconds.parse::<i64>() {
                Ok(sec) => Ok(Command::Expire {
                    key: key.to_string(),
                    seconds: sec,
                }),
                Err(_) => Err("올바르지 않은 TTL 값 입니다.".to_string()),
            },

            ["TTL", key] => Ok(Command::Ttl {
                key: key.to_string(),
            }),

            // CLIENT SETNAME xxx
            ["CLIENT", "SETNAME", name] => Ok(Command::ClientSetName {
                name: name.to_string(),
            }),

            // CLIENT SETINFO xxx yyy
            ["CLIENT", "SETINFO", ..] => {
                // SETINFO는 여러 파라미터가 올 수 있으니 나머지 다 받기
                let data = parts[2..].join(" ");
                Ok(Command::ClientSetInfo { data })
            }

            // HELLO 2
            ["HELLO", ver] => {
                let version = ver.parse::<i64>().unwrap_or(2);
                Ok(Command::Hello { version })
            }

            // HELLO (기본 RESP2)
            ["HELLO"] => Ok(Command::Hello { version: 2 }),

            ["INFO"] => Ok(Command::Info),

            ["QUIT"] => Ok(Command::Quit),

            ["EXIT"] | ["exit"] => Ok(Command::Exit),

            ["PING"] => Ok(Command::Ping),

            [] => Err("빈 명령어 입니다.".to_string()),

            _ => Ok(Command::Unknown),
        }
    }

    // 명령어를 실행하는 함수
    pub fn execute(&self, store: &Store) -> String {
        match self {
            Command::Set { key, value } => store.set(key.as_str(), value.as_str()),

            Command::Get { key } => match store.get(key.as_str()) {
                Some(value) => value,
                None => "(nil)".to_string(),
            },

            Command::Del { key } => store.del(key.as_str()).to_string(),

            Command::Expire { key, seconds } => store.expire(key.as_str(), *seconds).to_string(),

            Command::Ttl { key } => store.ttl(key.as_str()).to_string(),

            Command::Ping => "PONG".to_string(),

            Command::ClientSetName { .. } => "OK".to_string(),

            Command::ClientSetInfo { .. } => "OK".to_string(),

            Command::Hello { version } => {
                format!("server,redis,proto,{},role,master,id,1", version)
            }

            Command::Info => "# Server\r\nredis_version:7.0.0\r\nrole:master\r\n".to_string(),

            Command::Quit => "OK".to_string(),

            Command::Exit => "OK".to_string(),

            Command::Unknown => "ERR Unknown command".to_string(),
        }
    }
}
