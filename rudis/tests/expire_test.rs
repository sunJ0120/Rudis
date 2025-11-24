#[cfg(test)]
mod tests {
    use rudis::Store;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_expire_nonexistent() {
        let store = Store::new();
        assert_eq!(store.expire("not_exist", 10), 0);
    }

    #[test]
    fn test_ttl_nonexistent() {
        let store = Store::new();
        assert_eq!(store.ttl("not_exist"), -2);
    }

    #[test]
    fn test_ttl_after_expire() {
        let store = Store::new();
        store.set("key", "rudis");
        store.expire("key", 1);

        thread::sleep(Duration::from_secs(2));
        assert_eq!(store.ttl("key"), 0); // Lazy deletion이라 0이 나와야 한다.
    }

    #[test]
    fn test_ttl_with_expire() {
        let store = Store::new();
        store.set("key", "rudis");
        store.expire("key", 10);

        let ttl = store.ttl("key");
        assert!(ttl > 0 && ttl <= 10);
    }
}
