#[derive(Debug, PartialEq)]
pub enum RequestType {
    WorkingFlow,
    GlobalAction,
    Configurations,
    Resources,
    BlipFunction,
    Router
}