pub trait ApplicationRequests {
    fn get_auth_key(&self, identifier: &str);
}