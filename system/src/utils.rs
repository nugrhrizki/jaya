pub trait TemplateUtils {
    fn greet(string: &str) -> String {
        format!("Hello, {}!", string)
    }
}
