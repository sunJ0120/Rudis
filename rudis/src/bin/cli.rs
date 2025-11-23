use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Rudis CLI v0.1.0");
    println!("서버에 연결 중... 127.0.0.1:6379");

    let mut stream = match TcpStream::connect("127.0.0.1:6379").await {
        Ok(s) => {
            println!("🥳 서버 연결 성공!");
            println!("종료를 원하시면 'EXIT'를 눌러주세요.");
            println!();
            s
        },
        Err(e) => {
            eprintln!("😭 서버 연결 실패: {}", e);
            return Err(e.into());
        }
    };

    loop {
        // 기본 프롬프터 출력
        print!("rudis> ");
        io::stdout().flush().unwrap();

        // 입력 받기
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("😭 입력 하신 것을 읽는데 실패하였습니다. : {}", e);
                continue;
            }
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Exit 처리
        if input.eq_ignore_ascii_case("EXIT") {
            println!("🦀 Rudis CLI를 종료합니다.");
            break;
        }

        // 서버로 명령 전송
        if let Err(e) = stream.write_all(format!("{}\n", input).as_bytes()).await {
            eprintln!("😭 서버로 명령 전송 실패: {}", e);
            break;
        }

        // 서버 응답 읽기
        let mut buf = vec![0; 1024];
        let n = match stream.read(&mut buf).await {
            Ok(0) => {
                eprintln!("🦀 서버 연결이 종료되었습니다.");
                break;
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("😭 서버 응답 읽기 실패: {}", e);
                break;
            }
        };

        let response = String::from_utf8_lossy(&buf[..n]);
        print_response(&response);
    }

    Ok(())
}

fn print_response(resp: &str) {
    let lines: Vec<&str> = resp.split("\r\n").collect();

    if lines.is_empty() {
        return;
    }

    let first = lines[0];

    if first.starts_with('+') {
        // +OK
        println!("{}", &first[1..]);
    } else if first.starts_with('-') {
        // -ERR message
        println!("{}", first);
    } else if first.starts_with('$') {
        if first == "$-1" {
            println!("(nil)");
        } else {
            // Bulk String
            if lines.len() > 1 {
                println!("{}", lines[1]);
            }
        }
    } else {
        println!("{}", resp.trim());
    }
}