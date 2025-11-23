use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use rudis::{Command, RespValue, Store};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("🦀 Rudis 서버가 127.0.0.1:6379 에서 실행 중입니다.");

    let store = Arc::new(Store::new());

    loop{
        let (socket, addr) = listener.accept().await?;
        println!("🥳 새로운 연결: {}", addr);

        let store_clone = Arc::clone(&store);

        tokio::spawn(async move {
            handle_connection(socket, store_clone).await;
        });
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    store: Arc<Store>
){
    let mut buf = vec![0; 1024];

    loop{
        let n = match socket.read(&mut buf).await {
            Ok(0) => {
                println!("🦀 클라이언트 연결이 종료되었습니다.");
                return;
            },
            Ok(n) => n,
            Err(e) => {
                eprintln!("😭 소켓에서 데이터를 읽는데 실패하였습니다: {}", e);
                return;
            }
        };

        let input = String::from_utf8_lossy(&buf[..n]);
        let input = input.trim();

        // RESP 형식인지 확인하고 파싱 하도록 지원
        let _command_string = if input.starts_with('*') {
            match RespValue::parse(input) {
                Ok(resp) => {
                    match resp.to_command_string() {
                        Ok(cmd) => cmd,
                        Err(e) => {
                            let error_resp = format!("-ERR {}\r\n", e);
                            let _ = socket.write_all(error_resp.as_bytes()).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    let error_resp = format!("-ERR RESP 파싱 실패: {}\r\n", e);
                    let _ = socket.write_all(error_resp.as_bytes()).await;
                    continue;
                }
            }
        } else {
            println!("📝 일반 텍스트 명령");
            input.to_string()
        };

        // 기존의 Command 활용해서 파싱
        match Command::parse(&_command_string) {
            Ok(cmd) => {
                let response = cmd.execute(&store);
                let resp_response = to_resp_format(&response);

                if let Err(e) = socket.write_all(resp_response.as_bytes()).await {
                    eprintln!("😭 소켓에 응답을 쓰는데 실패하였습니다: {}", e);
                    return;
                }
            }
            Err(e) => {
                let error_resp = format!("-ERR {}\r\n", e);
                if let Err(e) = socket.write_all(error_resp.as_bytes()).await {
                    eprintln!("😭 소켓에 에러 응답을 쓰는데 실패하였습니다: {}", e);
                    return;
                }
            }
        }
    }
}

// 응답을 RESP 형식으로 변환한다.
fn to_resp_format(response: &str) -> String {
    match response {
        "OK" => "+OK\r\n".to_string(),
        "(nil)" => "$-1\r\n".to_string(),
        s if s.starts_with("ERR") => format!("-{}\r\n", s),
        s => {
            format!("${}\r\n{}\r\n", s.len(), s)
        }
    }
}