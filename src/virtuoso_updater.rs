#[derive(Clone, Debug)]
pub enum Status {
    Waiting,
    Ready(String),
    Error(String),
}

pub enum State {
    WaitingCheck,
    WaitingPrepare,
    WaitingApply,
}

pub trait VirtuosoUpdater {
    fn check_update(&mut self);
    fn prepare_update(&mut self);
    fn apply_update(&mut self);
    fn get_state(&self) -> State;
    fn get_status(&mut self) -> Status;
}
