package sunj.be.controller;

import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import lombok.RequiredArgsConstructor;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import sunj.be.service.CacheService;

@RestController
@RequestMapping("/api/cache")
@Tag(name = "Cache API", description = "Redis 캐시 관리 API")
@RequiredArgsConstructor
public class CacheController {
    private final CacheService cacheService;

    @PostMapping("/{key}")
    @Operation(summary = "캐시 저장", description = "key-value를 Redis에 저장합니다")
    public String set(@PathVariable String key, @RequestBody String value){
        cacheService.set(key, value);
        return "OK";
    }

    @GetMapping("/{key}")
    @Operation(summary = "캐시 조회", description = "key로 value를 조회합니다")
    public String get(@PathVariable String key){
       String value = cacheService.get(key);
       return value != null ? value : "(nil)";
    }

    @DeleteMapping("/{key}")
    @Operation(summary = "캐시 삭제", description = "key를 삭제합니다")
    public Long delete(@PathVariable String key){
        return cacheService.delete(key) ? 1L : 0L;
    }

    @PutMapping("/{key}/expire")
    @Operation(summary = "TTL 설정", description = "캐시 만료 시간을 설정합니다")
    public Boolean expire(@PathVariable String key, @RequestParam int seconds){
        return cacheService.expire(key, seconds);
    }

    @GetMapping("/{key}/ttl")
    @Operation(summary = "TTL 조회", description = "캐시 만료 시간을 조회합니다")
    public Long getTtl(@PathVariable String key){
        return cacheService.getTtl(key);
    }
}
