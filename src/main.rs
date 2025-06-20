use std::sync::{Arc, Mutex};
use std::thread;

mod match_info;
mod modules;
mod virtuoso_config;
mod virtuoso_logger;

use crate::modules::VirtuosoModule;
use crate::virtuoso_config::VirtuosoConfig;
use match_info::MatchInfo;

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

#[cfg(feature = "gpio_frontend")]
mod gpio_frontend;

fn main() {
    #[cfg(feature = "video_recorder")]
    compile_error!("Video recorder feature is not implemented yet");

    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));
    let config: Arc<Mutex<VirtuosoConfig>> =
        Arc::new(Mutex::new(VirtuosoConfig::load_config(None)));

    let virtuoso_logger: virtuoso_logger::VirtuosoLogger =
        virtuoso_logger::VirtuosoLogger::new(Arc::clone(&config));

    let logger: virtuoso_logger::Logger = virtuoso_logger.get_logger("Main thread".to_string());

    #[cfg(feature = "console_backend")]
    let mut console_backend = console_backend::ConsoleBackend::new(Arc::clone(&match_info));

    #[cfg(feature = "legacy_backend")]
    let mut legacy_backend = legacy_backend::LegacyBackend::new(
        Arc::clone(&match_info),
        Arc::clone(&config),
        virtuoso_logger.get_logger("Legacy backend".to_string()),
    );

    #[cfg(feature = "gpio_frontend")]
    let mut gpio_frontend = gpio_frontend::GpioFrontend::new(
        Arc::clone(&match_info),
        virtuoso_logger.get_logger("Gpio frontend".to_string()),
    );

    #[cfg(feature = "slint_frontend")]
    let mut slint_frontend = slint_frontend::SlintFrontend::new(Arc::clone(&match_info));

    #[cfg(feature = "cyrano_server")]
    let mut cyrano_server = cyrano_server::CyranoServer::new(
        Arc::clone(&match_info),
        Arc::clone(&config),
        virtuoso_logger.get_logger("Cyrano server".to_string()),
    );

    let logger_thread = thread::spawn(move || {
        virtuoso_logger.run();
    });

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

    #[cfg(feature = "gpio_frontend")]
    let gpio_frontend_thread = thread::spawn(move || {
        gpio_frontend.run();
    });
    #[cfg(feature = "gpio_frontend")]
    logger.info("Gpio frontend started".to_string());

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
    if let Err(e) = legacy_backend_thread.join() {
        logger.error(format!(
            "Failed to join legacy backend thread, error: {e:?}"
        ));
    } else {
        logger.info("Legacy backend stopped".to_string());
    }

    #[cfg(feature = "gpio_frontend")]
    if let Err(e) = gpio_frontend_thread.join() {
        logger.error(format!(
            "Failed to join legacy backend thread, error: {e:?}"
        ));
    } else {
        logger.info("Legacy backend stopped".to_string());
    }

    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    if let Err(e) = slint_frontend_thread.join() {
        logger.error(format!(
            "Failed to join slint frontend thread, error: {e:?}"
        ));
    } else {
        logger.info("Slint frontend stopped".to_string());
    }

    #[cfg(feature = "console_backend")]
    if let Err(e) = console_backend_thread.join() {
        logger.error(format!(
            "Failed to join console backend thread, error: {e:?}"
        ));
    } else {
        logger.info("Console backend stopped".to_string());
    }

    #[cfg(feature = "cyrano_server")]
    if let Err(e) = cyrano_server_thread.join() {
        logger.error(format!("Failed to join cyrano server thread, error: {e:?}"));
    } else {
        logger.info("Cyrano server stopped".to_string());
    }

    logger.info("Exiting program".to_string());

    logger.stop_logger();
    if let Err(e) = logger_thread.join() {
        eprintln!("Failed to join logger thread server thread, error: {e:?}");
    }
}
