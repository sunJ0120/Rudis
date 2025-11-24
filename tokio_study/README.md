# ⚡ Tokio Study - 비동기 프로그래밍 학습

## 개요

Rudis 프로젝트의 **TCP 멀티클라이언트 서버(MVP2)** 구현을 위해 학습한 Tokio 비동기 런타임에 대한 기록입니다.

---

## 🎯 학습 목표

- Rust의 async/await 문법 이해
- Tokio 런타임의 작동 원리
- 멀티클라이언트 동시성 처리

---

## 🥸 블로그 정리 글

| URL                                                                                    | 완료 |
|----------------------------------------------------------------------------------------|------|
| [🦀 Rudis 프로젝트 : Phase 2. Tokio 학습하기 - 튜토리얼](https://hot-and-spicy0120.tistory.com/42) | ✅ |
| [🦀 Rudis 프로젝트 : Phase 2. Tokio의 동시성 다루기](https://hot-and-spicy0120.tistory.com/45)                                           | ✅ |

---

## 🚀 실제 적용

이 학습 내용은 **[Rudis TCP 서버](../rudis/bin/server.rs)**에 다음과 같이 적용되었습니다.

- Tokio 멀티스레드 런타임 활용
- 클라이언트별 독립적인 태스크 생성
- Arc<Mutex> 기반 상태 공유

---

## 📂 파일 구조

```
tokio_study/
├── src/
│   ├── async_ex.rs                   # 비동기 적용한 쓰레드 시간 측정
│   ├── main.rs                       # 진입점
│   ├── mini_redis_server_ex.rs       # 문서에 있는 mini_redis 실행 예시
│   ├── spawning.rs                   # tokio::spawn 실습
│   └── spawning_set_get.rs           # set, get 구성 실습
├── Cargo.toml
└── README.md
```

---

## 🔗 관련 프로젝트

- **[Rudis](../rudis/)** : 이 학습을 바탕으로 완성한 Redis 클론