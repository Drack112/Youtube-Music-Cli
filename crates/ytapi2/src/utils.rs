pub trait StringUtils {
    fn between(&self, start: &str, end: &str) -> Option<&str>;
}

impl StringUtils for String {
    fn between(&self, start: &str, end: &str) -> Option<&str> {
        let string: &str = &self[self.find(start)? + start.len()..];
        Some(&string[..string.find(end)?])
    }
}
