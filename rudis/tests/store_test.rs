#[cfg(test)]
mod tests {
    use rudis::Store;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_set() {
        let store = Store::new();
        store.set("key", "rudis");
        assert_eq!(store.get("key"), Some("rudis".to_string()));
    }

    #[test]
    fn test_set_init_expire() {
        let store = Store::new();
        store.set("key", "rudis");
        assert_eq!(store.ttl("key"), -1);
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = Store::new();
        let result = store.get("not_exist");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_expired_key() {
        let store = Store::new();
        store.set("key", "rudis");
        store.expire("key", 1);

        assert_eq!(store.get("key"), Some("rudis".to_string()));
        thread::sleep(Duration::from_secs(2));
        assert_eq!(store.get("key"), None);
    }

    #[test]
    fn test_del_existing_key() {
        let store = Store::new();
        store.set("key", "rudis");

        let result = store.del("key");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_del_nonexistent_key() {
        let store = Store::new();
        let result = store.del("not_exist");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_expire_basic() {
        let store = Store::new();
        store.set("key", "rudis");

        assert_eq!(store.expire("key", 10), 1);
    }
}
