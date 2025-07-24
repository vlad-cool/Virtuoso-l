use std::sync::{Arc, Mutex};
use std::thread;

mod hw_config;
mod match_info;
mod modules;
mod virtuoso_config;
mod virtuoso_logger;

use crate::hw_config::HardwareConfig;
use crate::modules::VirtuosoModule;
use crate::virtuoso_config::VirtuosoConfig;
use match_info::MatchInfo;

#[cfg(feature = "cyrano_server")]
#[path = "../private_modules/cyrano/cyrano_server.rs"]
mod cyrano_server;

#[cfg(feature = "gpio-cdev")]
mod gpio;

#[cfg(feature = "legacy_backend")]
mod legacy_backend;

#[cfg(feature = "console_backend")]
mod console_backend;

#[cfg(feature = "sdl_frontend")]
mod sdl_frontend;

#[cfg(feature = "gpio_frontend")]
mod gpio_frontend;

#[cfg(feature = "repeater")]
mod repeater;

fn main() {
    #[cfg(feature = "video_recorder")]
    compile_error!("Video recorder feature is not implemented yet");

    /*
    TODO Properly swap sides
    TODO Cyrano softer error
    TODO Cyrano
    TODO Repeater ACK / NAK
    TODO Repeater auto role
    TODO Repeater reorder receiver
    TODO Menu
    TODO Replace mutex with rwlock
     */

    let config: Arc<Mutex<VirtuosoConfig>> = Arc::new(Mutex::new(VirtuosoConfig::load_config()));

    let virtuoso_logger: virtuoso_logger::VirtuosoLogger =
        virtuoso_logger::VirtuosoLogger::new(Arc::clone(&config));

    let logger: virtuoso_logger::Logger = virtuoso_logger.get_logger("Main thread".to_string());

    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));

    let hw_config: HardwareConfig = HardwareConfig::get_config(&logger);

    #[cfg(feature = "console_backend")]
    let console_backend = console_backend::ConsoleBackend::new(Arc::clone(&match_info));

    #[cfg(feature = "legacy_backend")]
    let legacy_backend: legacy_backend::LegacyBackend = legacy_backend::LegacyBackend::new(
        Arc::clone(&match_info),
        Arc::clone(&config),
        virtuoso_logger
            .get_logger("Legacy backend".to_string())
            .enable_debug(),
    );

    #[cfg(feature = "gpio_frontend")]
    let gpio_frontend: gpio_frontend::GpioFrontend = gpio_frontend::GpioFrontend::new(
        Arc::clone(&match_info),
        virtuoso_logger.get_logger("Gpio frontend".to_string()),
    );

    #[cfg(feature = "sdl_frontend")]
    let sdl_frontend: sdl_frontend::SdlFrontend = sdl_frontend::SdlFrontend::new(
        Arc::clone(&match_info),
        hw_config.clone(),
        virtuoso_logger.get_logger("sdl frontend".to_string()),
    );

    #[cfg(feature = "cyrano_server")]
    let cyrano_server = cyrano_server::CyranoServer::new(
        Arc::clone(&match_info),
        Arc::clone(&config),
        virtuoso_logger.get_logger("Cyrano server".to_string()),
    );

    #[cfg(feature = "repeater")]
    let repeater = repeater::Repeater::new(
        Arc::clone(&match_info),
        virtuoso_logger.get_logger("Repeater".to_string()),
        hw_config.clone(),
    );
    #[cfg(feature = "repeater")]
    match &repeater {
        Ok(_) => {}
        Err(err) => {
            logger.critical_error(format!("Failed to create repeater, error: {err}"));
        }
    }

    let logger_thread: thread::JoinHandle<()> = thread::spawn(move || {
        virtuoso_logger.run();
    });

    #[cfg(feature = "console_backend")]
    let console_backend_thread: thread::JoinHandle<()> = thread::spawn(move || {
        console_backend.run();
    });
    #[cfg(feature = "console_backend")]
    logger.info("Console backend started".to_string());

    #[cfg(feature = "legacy_backend")]
    let legacy_backend_thread: thread::JoinHandle<()> = thread::spawn(move || {
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

    #[cfg(feature = "repeater")]
    let repeater_thread = if let Ok(repeater) = repeater {
        let thread = thread::spawn(move || {
            repeater.run();
        });
        logger.info("Repeater started".to_string());
        thread
    } else {
        logger.critical_error("Repeater did not start because it did not exist".to_string());
        thread::spawn(move || {})
    };

    // };
    // #[cfg(feature = "repeater")]

    #[cfg(feature = "sdl_frontend")]
    {
        use crate::match_info::ProgramState;

        logger.info("sdl frontend started in main thread".to_string());
        sdl_frontend.run();
        logger.info("sdl frontend stopped in main thread".to_string());
        match_info.lock().unwrap().program_state = ProgramState::Exiting;
    }

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

    #[cfg(feature = "repeater")]
    if let Err(e) = repeater_thread.join() {
        logger.error(format!("Failed to join repeater thread, error: {e:?}"));
    } else {
        logger.info("Cyrano server stopped".to_string());
    }

    logger.info("Exiting program".to_string());

    logger.stop_logger();
    if let Err(e) = logger_thread.join() {
        eprintln!("Failed to join logger thread server thread, error: {e:?}");
    }
}
