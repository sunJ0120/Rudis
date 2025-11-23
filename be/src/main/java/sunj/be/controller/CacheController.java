package sunj.be.controller;

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
@RequiredArgsConstructor
public class CacheController {
    private final CacheService cacheService;

    @PostMapping("/{key}")
    public String set(@PathVariable String key, @RequestBody String value){
        cacheService.set(key, value);
        return "OK";
    }

    @GetMapping("/{key}")
    public String get(@PathVariable String key){
       String value = cacheService.get(key);
       return value != null ? value : "(nil)";
    }

    @DeleteMapping("/{key}")
    public Long delete(@PathVariable String key){
        return cacheService.delete(key) ? 1L : 0L;
    }

    @PutMapping("/{key}/expire")
    public Boolean expire(@PathVariable String key, @RequestParam int seconds){
        return cacheService.expire(key, seconds);
    }

    @GetMapping("/{key}/ttl")
    public Long getTtl(@PathVariable String key){
        return cacheService.getTtl(key);
    }
}
