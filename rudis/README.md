# 🦀 Rudis - Redis Clone in Rust

## 프로젝트 개요

![Skills](https://skillicons.dev/icons?i=rust)

Rust를 학습하면서 느낀 점은, Rust가 기존 C++과 C언어의 여러 단점들을 컴파일러 선에서 효율적으로 처리하고 있다는 부분이었습니다. 과연 이 언어의 장점이 어디서 잘 발휘될 수 있을까 궁금해졌고, **직접 Redis 클론을 구현해보기로** 결정했습니다.

### 기술 스택

> Tokio 비동기 런타임, Arc<Mutex> 기반 스레드 안전 상태 관리, RESP 프로토콜 구현, Docker, CI/CD

### 목표

- Rust의 메모리 안전성이 멀티클라이언트 환경에서 어떻게 작동하는지 직접 체감하기
- Prometheus & Grafana를 통해 실시간으로 모니터링하면서 시스템 안정성을 검증하기

---

## 🥳 MVP 진행 상황

| MVP | 단계 | 상태 | 블로그 글                                                                                                                                                                                                                                                                                                                             |
|-----|------|------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **MVP1** | CLI 기반 인메모리 DB | ✅ | [🦀 Rudis 프로젝트 : 기획](https://hot-and-spicy0120.tistory.com/38)<br>[🦀 Rudis 프로젝트 : Redis 찍먹하면서 요구사항 정리하기](https://hot-and-spicy0120.tistory.com/39)<br>[🦀 Rudis 프로젝트 : Phase 1. 기본 명령어와 test 구현하기](https://hot-and-spicy0120.tistory.com/40)<br>[🦀 Rudis 프로젝트 : Phase 1. CLI DB 구현하기](https://hot-and-spicy0120.tistory.com/44) |
| **MVP2** | TCP 기반 멀티클라이언트 서버 | ✅ | [🦀 Rudis 프로젝트 : Phase 2. TCP 서버 구축하기 & Phase 2 목표 정리](https://hot-and-spicy0120.tistory.com/41)<br>[🦀 Rudis 프로젝트 : Phase 2. Rudis TCP 서버 구현하기](https://hot-and-spicy0120.tistory.com/46)                                                                                                                               |
| **MVP3** | RESP 프로토콜 + Spring Boot 연동 + Docker + CI/CD + Prometheus & Grafana | 🔄 |                                                                                                                                                                                                                                                                                                                           |

---

## 🚀 빠른 시작

### 설치 & 실행
```bash
docker compose up -d
```

### 사용
```bash
redis-cli -h localhost -p 6379
> PING
> SET key value
> GET key
```

#### Rudis와 함께 전체 스택 실행
```bash
docker compose up -d
# Rudis: localhost:6379
# Spring Boot: localhost:8090
# Prometheus: localhost:9090
# Grafana: localhost:3000
```

---

## 📌 지원 명령어

### Key-Value 작업

| 명령어 | 설명 | 예시 |
|--------|------|------|
| `SET` | 키에 값 저장 | `SET mykey "Hello"` |
| `GET` | 키의 값 조회 | `GET mykey` |
| `DEL` | 키 삭제 | `DEL mykey` |
| `EXPIRE` | 키에 만료 시간 설정 (초) | `EXPIRE mykey 60` |
| `TTL` | 키의 남은 시간 조회 | `TTL mykey` |

### 서버 명령어

| 명령어 | 설명 | 예시 |
|--------|------|------|
| `PING` | 서버 연결 확인 | `PING` |
| `INFO` | 서버 정보 조회 | `INFO` |
| `HELLO` | 프로토콜 버전 협상 | `HELLO 3` |
| `QUIT` | 연결 종료 | `QUIT` |

### 클라이언트 명령어

| 명령어 | 설명 | 예시 |
|--------|------|------|
| `CLIENT SETNAME` | 클라이언트 이름 설정 | `CLIENT SETNAME myclient` |
| `CLIENT SETINFO` | 클라이언트 정보 설정 | `CLIENT SETINFO lib-name "Lettuce"` |

---

## 🌟 성과

### 기술적 구현
- ✅ RESP2 프로토콜 완전 호환 구현 (Redis 표준 준수)
- ✅ Tokio 기반 멀티클라이언트 TCP 서버 (동시성 처리)
- ✅ Arc<Mutex>를 통한 스레드 안전 상태 관리
- ✅ 명령어 파싱 및 실행 로직 (SET, GET, DEL, EXPIRE, TTL 등)

### 실제 활용성
- ✅ Spring Data Redis (Lettuce 클라이언트)와의 seamless 연동
- ✅ CLI 모드 (MVP1)와 Server 모드 (MVP2) 동시 지원
- ✅ Docker 컨테이너화로 즉시 배포 가능
- ✅ GitHub Actions CI/CD 파이프라인 구축

###  모니터링 & 운영
- ✅ Docker Compose로 전체 스택 통합 관리
<!-- - ✅ Prometheus & Grafana를 통한 실시간 성능 모니터링 -->


## 📂 파일 구조
```
rudis/
├── src/
│   ├── protocols.rs            # RESP 프로토콜 구현 
│   ├── lib.rs                  # 라이브러리 모듈 선언
│   ├── command.rs              # cmd 명령어 실행 로직
│   ├── store.rs                # Arc<Mutex> 기반 인메모리 저장소
│   └── bin/              
│       ├── server.rs           # TCP 멀티클라이언트 서버 (Tokio 기반)
│       └── cli.rs              # 로컬 CLI 인터페이스 (MVP1)
├── tests/                      # 통합 & 단위 테스트
│   ├── command_parse_test.rs   # 명령어 파싱 테스트
│   ├── command_execute_test.rs # 명령어 실행 테스트
│   ├── protocol_test.rs        # RESP 프로토콜 테스트
│   ├── expire_test.rs          # 키 만료 시간 테스트
│   └── store_test.rs           # 저장소 동시성 테스트
│
├── Dockerfile                  # 컨테이너 빌드 설정
├── Cargo.toml                  # Rust 의존성 및 프로젝트 설정
└── README.md                   # 프로젝트 문서
```