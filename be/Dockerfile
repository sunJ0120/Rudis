FROM gradle:8-jdk21 as builder

WORKDIR /app

COPY . .

RUN chmod +x gradlew

RUN ./gradlew build -x test --no-daemon

FROM eclipse-temurin:21-jdk-alpine-3.22

WORKDIR /app

COPY --from=builder /app/build/libs/*.jar app.jar

EXPOSE 8090

CMD ["java", "-jar", "app.jar"]