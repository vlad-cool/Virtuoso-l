use std::sync::{Arc, Mutex};
use std::thread;

mod match_info;
mod modules;
mod virtuoso_config;
mod virtuoso_logger;

use crate::modules::VirtuosoModule;
use crate::virtuoso_config::VirtuosoConfig;
use match_info::MatchInfo;
// use log::{info, warn, error, debug, trace};

#[cfg(feature = "cyrano_server")]
mod cyrano_server;

#[cfg(feature = "legacy_backend")]
mod gpio;
#[cfg(feature = "legacy_backend")]
mod legacy_backend;

#[cfg(feature = "console_backend")]
mod console_backend;

#[cfg(feature = "slint_frontend")]
mod layouts;
#[cfg(feature = "slint_frontend")]
mod slint_frontend;

fn main() {
    // env_logger::init();
    #[cfg(feature = "video_recorder")]
    todo!();

    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));
    let config: Arc<Mutex<VirtuosoConfig>> =
        Arc::new(Mutex::new(VirtuosoConfig::load_config(None)));

    let virtuoso_logger = virtuoso_logger::VirtuosoLogger::new(Arc::clone(&config));

    let logger: virtuoso_logger::Logger = virtuoso_logger.get_logger("Main thread".to_string());

    let logger_thread = thread::spawn(move || {
        virtuoso_logger.run();
    });

    #[cfg(feature = "console_backend")]
    let mut console_backend = console_backend::ConsoleBackend::new(Arc::clone(&match_info));

    #[cfg(feature = "legacy_backend")]
    let mut legacy_backend =
        legacy_backend::LegacyBackend::new(Arc::clone(&match_info), Arc::clone(&config));

    #[cfg(feature = "slint_frontend")]
    let mut slint_frontend = slint_frontend::SlintFrontend::new(Arc::clone(&match_info));

    #[cfg(feature = "cyrano_server")]
    let mut cyrano_server =
        cyrano_server::CyranoServer::new(Arc::clone(&match_info), Arc::clone(&config));

    #[cfg(feature = "console_backend")]
    let console_backend_thread = thread::spawn(move || {
        console_backend.run();
    });
    #[cfg(feature = "console_backend")]
    logger.info("Console backend started".to_string());

    #[cfg(feature = "legacy_backend")]
    let legacy_backend_thread = thread::spawn(move || {
        legacy_backend.run();
    });
    #[cfg(feature = "legacy_backend")]
    logger.info("Legacy backend started".to_string());

    #[cfg(feature = "cyrano_server")]
    let cyrano_server_thread = thread::spawn(move || {
        cyrano_server.run();
    });
    #[cfg(feature = "cyrano_server")]
    logger.info("Cyrano server started".to_string());

    #[cfg(feature = "slint_frontend")]
    #[cfg(target_os = "macos")]
    logger.info("Slint frontend started in main thread".to_string());
    #[cfg(feature = "slint_frontend")]
    #[cfg(target_os = "macos")]
    slint_frontend.run();
    #[cfg(feature = "slint_frontend")]
    #[cfg(target_os = "macos")]
    logger.info("Slint frontend stopped in main thread".to_string());

    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    let slint_frontend_thread = thread::spawn(move || {
        slint_frontend.run();
    });
    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    logger.info("Slint frontend started".to_string());

    #[cfg(feature = "legacy_backend")]
    legacy_backend_thread.join().unwrap();
    #[cfg(feature = "legacy_backend")]
    logger.info("Legacy backend stopped".to_string());

    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    slint_frontend_thread.join().unwrap();

    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    logger.info("Slint frontend stopped".to_string());

    #[cfg(feature = "console_backend")]
    console_backend_thread.join().unwrap();
    #[cfg(feature = "console_backend")]
    logger.info("Console backend stopped".to_string());

    #[cfg(feature = "cyrano_server")]
    {
        cyrano_server_thread.join().unwrap();
        logger.info("Cyrano server stopped".to_string());
    }

    logger.info("Exiting program".to_string());

    logger.stop_logger();
    logger_thread.join().unwrap();
}
