// RESP 프로토콜 파싱
pub enum RespValue {
    SimpleString(String),
    BulkString(String),
    Integer(i64),
    Array(Vec<RespValue>),
    Null,
}

impl RespValue {
    pub fn parse(input: &str) -> Result<RespValue, String> {
        if input.is_empty() {
            return Err("빈 입력입니다.".to_string());
        }

        let first_char = input.chars().next().unwrap();

        match first_char {
            '*' => Self::parse_array(input),
            '$' => Self::parse_bulk_string(input),
            '+' => Self::parse_simple_string(input),
            _ => Err(format!("지원하지 않는 RESP 타입입니다: {}", first_char)),
        }
    }

    // Array 파싱: *3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
    fn parse_array(input: &str) -> Result<Self, String> {
        let lines: Vec<&str> = input
            .split("\r\n")
            .filter(|s| !s.is_empty()) // ← 빈 줄 제거
            .collect();

        if lines.is_empty() {
            return Err("잘못된 Array 형식입니다.".to_string());
        }

        let count_str = &lines[0][1..];
        let count: usize = count_str
            .parse()
            .map_err(|_| "Array 개수 파싱 실패".to_string())?;

        let mut elements = Vec::new();
        let mut line_idx = 1;

        for _ in 0..count {
            if line_idx >= lines.len() {
                return Err("Array 요소가 부족합니다.".to_string());
            }

            // Bulk String 파싱: $3\r\nSET
            if !lines[line_idx].starts_with('$') {
                return Err(format!(
                    "Array 요소는 Bulk String이어야 합니다. 받은 것: {}",
                    lines[line_idx]
                ));
            }

            let len_str = &lines[line_idx][1..];
            let bulk_len: usize = len_str
                .parse()
                .map_err(|_| "Bulk String 길이 파싱 실패".to_string())?;

            line_idx += 1;
            if line_idx >= lines.len() {
                return Err("Bulk String 값이 없습니다.".to_string());
            }

            let value = lines[line_idx].to_string();

            // 길이가 정확한지 확인
            if value.len() != bulk_len {
                eprintln!("⚠️ 길이 불일치: 예상 {}, 실제 {}", bulk_len, value.len());
            }

            elements.push(RespValue::BulkString(value));
            line_idx += 1;
        }

        Ok(RespValue::Array(elements))
    }

    fn parse_bulk_string(input: &str) -> Result<Self, String> {
        let lines: Vec<&str> = input.split("\r\n").filter(|s| !s.is_empty()).collect();

        if lines.is_empty() {
            return Err("잘못된 Bulk String 형식입니다.".to_string());
        }

        let len_str = &lines[0][1..];
        if len_str == "-1" {
            // Null 체크를 앞에 둔다.
            return Ok(RespValue::Null);
        }

        if lines.len() < 2 {
            return Err("잘못된 Bulk String 형식입니다.".to_string());
        }

        Ok(RespValue::BulkString(lines[1].to_string()))
    }

    fn parse_simple_string(input: &str) -> Result<Self, String> {
        let content = input
            .trim_start_matches('+')
            .trim_end_matches("\r\n")
            .to_string();
        Ok(RespValue::SimpleString(content))
    }

    pub fn to_command_string(&self) -> Result<String, String> {
        match self {
            RespValue::Array(elements) => {
                let strings: Result<Vec<String>, String> = elements
                    .iter()
                    .map(|e| match e {
                        RespValue::BulkString(s) => Ok(s.clone()),
                        _ => Err("Array 요소가 BulkString이 아닙니다.".to_string()),
                    })
                    .collect();

                strings.map(|v| v.join(" "))
            }
            _ => Err("Array만 Command로 변환 가능합니다.".to_string()),
        }
    }
}
