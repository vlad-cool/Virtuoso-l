pub struct VirtuosoModuleContext {
    pub logger: crate::virtuoso_logger::Logger,
    pub config: crate::VirtuosoConfig,
    pub hw_config: crate::HardwareConfig,
}

pub trait VirtuosoModule {
    fn run(self);
}
