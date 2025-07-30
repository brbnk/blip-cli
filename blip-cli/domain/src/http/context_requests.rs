pub trait ContextRequests {
    fn get_context(&self, identifier: &str, contact: &str, context: &str);
}