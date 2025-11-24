# 🍃 Spring Boot - Rudis 통합 테스트 앱

## 개요

![Tech Stack](https://skillicons.dev/icons?i=spring)

Rudis(Redis 클론)를 실제 Spring Boot 애플리케이션에서 사용할 수 있는지 검증하고,
Docker 컨테이너 환경에서의 안정성을 확인하는 앱입니다.

---

## 🎯 목표

- Spring Data Redis 클라이언트(Lettuce)로 Rudis 실제 연동 검증
- Docker Compose 환경에서 멀티 서비스 통합 테스트
- 실제 사용 환경에서의 호환성 확인

---

## 🚀 빠른 시작

### Rudis와 함께 실행

```bash
cd ..  # 프로젝트 루트
docker compose up -d

# 헬스 체크
curl http://localhost:8090/actuator/health
```

## 🔌 API 엔드포인트

Rudis와 연동하는 REST API입니다. 자세한 내용은 **Swagger UI**에서 확인하세요.

| 메서드 | 경로 | 설명 |
|--------|------|------|
| `POST` | `/api/cache/{key}` | 캐시 저장 (SET) |
| `GET` | `/api/cache/{key}` | 캐시 조회 (GET) |
| `DELETE` | `/api/cache/{key}` | 캐시 삭제 (DEL) |
| `PUT` | `/api/cache/{key}/expire` | TTL 설정 (EXPIRE) |
| `GET` | `/api/cache/{key}/ttl` | TTL 조회 (TTL) |

**🔗 [Swagger UI에서 테스트하기](http://localhost:8090/swagger-ui.html)**


## 📂 파일 구조
```
be/
├── src/main/java/sunj/be/
│   ├── controller/       # REST API
│   ├── service/          # 비즈니스 로직
│   └── config/           # Redis 설정
├── src/main/resources/
│   └── application.yml   # Spring & Redis 설정
├── build.gradle
├── Dockerfile
└── README.md
```

---

## 📊 모니터링 (예정)

- Prometheus 메트릭 수집 예정
- Grafana 대시보드 구축 예정