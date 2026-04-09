pub trait MirrorRequests {
    fn get_working_flow(&self, tenant: &str, identifier: &str);
    fn get_working_subflow(&self, tenant: &str, flow_id: &str);
    fn get_resources(&self, tenant: &str, identifier: &str);
    fn get_config_variables(&self, tenant: &str, identifier: &str);
    fn get_global_actions(&self, tenant: &str, identifier: &str);
    fn get_blip_functions(&self, tenant: &str, identifier: &str);
    fn get_router_children(&self, tenant: &str, identifier: &str, tier: &str) -> String;
}