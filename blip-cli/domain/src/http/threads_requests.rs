pub trait ThreadsRequests {
    fn get_thread(&self, identifier: &str, contact: &str);
}