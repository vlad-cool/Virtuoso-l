// use std::sync::{Arc, Mutex};

// pub use crate::virtuoso_logger::Logger;
// pub use crate::virtuoso_config::VirtuosoConfig;
// pub use crate::hw_config::HardwareConfig;
// pub use crate::match_info::MatchInfo;

// #[derive(Clone)]
// pub struct VirtuosoModuleContext {
//     pub logger: Logger,
//     pub config: Arc<Mutex<VirtuosoConfig>>,
//     pub hw_config: HardwareConfig,
//     pub match_info: Arc<Mutex<MatchInfo>>,
// }

// pub trait ConstructVirtuosoModule {
//     fn new(context: VirtuosoModuleContext) -> Self;
// }

pub trait VirtuosoModule {
    fn run(self);
}
