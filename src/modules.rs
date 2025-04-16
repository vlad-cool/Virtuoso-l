#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum Modules {
    #[cfg(feature = "cyrano_server")]
    CyranoServer,
    #[cfg(feature = "console_backend")]
    ConsoleBackend,
    #[cfg(feature = "legacy_backend")]
    LegacyBackend,
    #[cfg(feature = "slint_frontend")]
    SlintFrontend,
    #[cfg(feature = "video_recorder")]
    VideoRecorder,
}

pub trait VirtuosoModule {
    fn run(&mut self);
    fn get_module_type(&self) -> Modules;
}
