# 🦀 rust_study

러스트 공식문서로 문법 & 소유권 개념 공부하기

![Skills](https://skillicons.dev/icons?i=rust)

--------------

## 📚 이 폴더의 역할

The Rust Programming Language 공식 문서를 따라 **Rust 핵심 개념을 학습**한 기록입니다.
이 학습의 결과물은 `../rudis/` 프로젝트에서 확인할 수 있습니다.


## 🥳 성과 : 학습으로 끝나지 않고 실제 프로젝트로

이 학습을 통해 얻은 지식으로 **Rudis(Redis 클론)** 프로젝트를 완성했습니다.
- Ownership & Borrowing 이해 ✅
- Tokio 비동기 프로그래밍 ✅ (별도 tokio_study/ 참고)
- TCP 서버, RESP 프로토콜 구현 ✅


## 🎓 날짜 별 학습 내용

| 날짜                 | 학습 내용                                                         | 완료 |
|--------------------|---------------------------------------------------------------|------|
| 2025-11-07         | Chapter 0: Rust 알아보기<br>Chapter 1: Rust 설치, cargo & rustc 알아보기 | ✅ |
| 2025-11-08         | Chapter 2: 간단한 추리 게임 만들기<br>Chapter 3: 프로그래밍 개념 익히기           | ✅ |
| 2025-11-09         | 🌟Chapter 4: 소유권 이해하기                                         | ✅ |
| 2025-11-10 ~ 2025-11-13 | Chapter 5: 구조체로 연관된 데이터 구조화하기                                 | ✅ |
| 2025-11-14 ~ 2025-11-15     | Chapter 8: HashMap, Vector, String 데이터 구조 익히기                 | ✅ |


## 🥸 블로그 정리 글

| 챕터  | URL                                                                          | 완료 |
|-----|------------------------------------------------------------------------------|------|
| 0️⃣ | [오픈미션 주제 고르기](https://hot-and-spicy0120.tistory.com/28)                      | ✅ |
| 0️⃣ | [러스트에 대해서](https://hot-and-spicy0120.tistory.com/29)                         | ✅ |
| 1️⃣ | [러스트 진짜 시작!](https://hot-and-spicy0120.tistory.com/30)                       | ✅ |
| 2️⃣ | [러스트로 간단한 번호 추리게임 만들면서 기본 문법을 익히기](https://hot-and-spicy0120.tistory.com/31) | ✅ |
| 3️⃣ | [러스트의 일반적인 프로그래밍 개념_변수와 기본타입](https://hot-and-spicy0120.tistory.com/32)      | ✅ |
| 3️⃣ | [러스트의 일반적인 프로그래밍 개념_함수와 제어 흐름](https://hot-and-spicy0120.tistory.com/33)     | ✅ |
| 4️⃣ | [소유권을 알아보자~](https://hot-and-spicy0120.tistory.com/34)                       | ✅ |
| 4️⃣ | [참조와 대여](https://hot-and-spicy0120.tistory.com/35)                           | ✅ |
| 4️⃣ | [슬라이스](https://hot-and-spicy0120.tistory.com/36)                             | ✅ |
| 5️⃣ | [구조체로 연관된 데이터를 구조화하기](https://hot-and-spicy0120.tistory.com/37)      | ✅ |


## 📁 파일 구조
```
rust_study/
├── src/                        # rustc가 src를 실행하는 실행점
│   ├── main.rs
│   ├── example2.rs             # 챕터2. 추리게임
│   ├── example3.rs             # 챕터3. 일반적인 프로그래밍 개념
│   ├── example3_test1.rs       # 챕터3-1. 섭씨 화씨 변환
│   ├── example3_test2.rs       # 챕터3-2. n번째 피보나치 수 구하기
│   ├── example4.rs             # 챕터4. 소유권 이해하기
│   ├── example5.rs             # 챕터5. 구조체로 연관된 데이터를 구조화하기
│   ├── example5_test1.rs       # 챕터5-1. 구조체 예제 프로그램
│   ├── example5_test2.rs       # 챕터5-2. 구조체 예제 프로그램 - 메서드 문법
│   ├── example8_1.rs           # 챕터8-1. Vec<T> 자료구조
│   ├── example8_2.rs           # 챕터8-2. String 자료구조와 UTF-8
│   └── example8_3.rs           # 챕터8-3. HashMap 자료구조
├── target/
├── .gitignore
├── Cargo.toml
└── README.md
```

-------

## 📖 참고 자료

- [The Rust Programming Language (공식 문서 번역본)](https://doc.rust-kr.org/ch01-03-hello-cargo.html)
