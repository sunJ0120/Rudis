package sunj.be.service;

import java.util.concurrent.TimeUnit;
import lombok.RequiredArgsConstructor;
import org.springframework.data.redis.core.RedisTemplate;
import org.springframework.stereotype.Service;

@Service
@RequiredArgsConstructor
public class CacheService {
    private final RedisTemplate<String, String> redisTemplate;

    public void set(String key, String value) {
        redisTemplate.opsForValue().set(key, value);
    }

    public String get(String key){
        return redisTemplate.opsForValue().get(key);
    }

    public Boolean delete(String key){
        return redisTemplate.delete(key);
    }

    public Boolean expire(String key, long seconds){
        return redisTemplate.expire(key, seconds, TimeUnit.SECONDS);
    }

    public Long getTtl(String key){
        return redisTemplate.getExpire(key, TimeUnit.SECONDS);
    }
}
