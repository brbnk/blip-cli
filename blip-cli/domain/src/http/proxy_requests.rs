pub trait ProxyRequests {
  fn get_builder_flow(&self);
  fn get_builder_resources(&self);
  fn get_builder_configurations(&self);
  fn get_builder_global_actions(&self);
  fn get_blip_functions(&self);
  fn get_router_chidlren(&self);
  fn get_key(&self);
  fn get_context(&self, contact: &str, context: &str);
  fn get_thread(&self, contact: &str);
}