use std::thread;

mod hw_config;
mod match_info;
mod modules;
mod settings_menu;
mod virtuoso_config;
mod virtuoso_logger;

use crate::hw_config::HardwareConfig;
use crate::modules::{SettingsMenu, VirtuosoModule, VirtuosoModuleContext};
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
    TODO wlr-randr (without run.sh)
    TODO Properly swap sides
    TODO Cyrano
    TODO Cyrano time handling
    TODO Repeater reorder receiver
    TODO Menu
    TODO Replace mutex with rwlock
    TODO Flash new score value if was updated automatically
     */

    /*
    Team score manual counting?
     */

    let config: VirtuosoConfig = VirtuosoConfig::load_config();

    let virtuoso_logger: virtuoso_logger::VirtuosoLogger =
        virtuoso_logger::VirtuosoLogger::new(&config);

    let logger: virtuoso_logger::Logger = virtuoso_logger.get_logger("Main thread".to_string());

    let match_info: MatchInfo = MatchInfo::new();

    let hw_config: HardwareConfig = HardwareConfig::get_config(&logger);

    let settings_menu: SettingsMenu = SettingsMenu::new();

    let context: VirtuosoModuleContext =
        VirtuosoModuleContext::new(logger, config, hw_config, match_info, settings_menu);

    #[cfg(feature = "console_backend")]
    let console_backend: console_backend::ConsoleBackend = console_backend::ConsoleBackend::new(
        context.with_logger(virtuoso_logger.get_logger("Console backend".to_string())),
    );

    #[cfg(feature = "legacy_backend")]
    let legacy_backend: legacy_backend::LegacyBackend = legacy_backend::LegacyBackend::new(
        context.with_logger(virtuoso_logger.get_logger("Legacy backend".to_string())),
    );

    #[cfg(feature = "gpio_frontend")]
    let gpio_frontend: gpio_frontend::GpioFrontend = gpio_frontend::GpioFrontend::new(
        context.with_logger(virtuoso_logger.get_logger("Gpio frontend".to_string())),
    );

    #[cfg(feature = "sdl_frontend")]
    let sdl_frontend: sdl_frontend::SdlFrontend = sdl_frontend::SdlFrontend::new(
        context.with_logger(virtuoso_logger.get_logger("sdl frontend".to_string())),
    );

    #[cfg(feature = "cyrano_server")]
    let cyrano_server: Result<cyrano_server::CyranoServer, String> =
        cyrano_server::CyranoServer::new(
            context.with_logger(virtuoso_logger.get_logger("Cyrano server".to_string())),
        );

    #[cfg(feature = "cyrano_server")]
    match &cyrano_server {
        Ok(_) => {}
        Err(err) => {
            context
                .logger
                .critical_error(format!("Failed to create cyrano server, error: {err}"));
        }
    }

    #[cfg(feature = "repeater")]
    let repeater: Result<repeater::Repeater, String> = repeater::Repeater::new(
        context.with_logger(virtuoso_logger.get_logger("Repeater".to_string())),
    );
    #[cfg(feature = "repeater")]
    match &repeater {
        Ok(_) => {}
        Err(err) => {
            context
                .logger
                .critical_error(format!("Failed to create repeater, error: {err}"));
        }
    }

    let logger_thread: thread::JoinHandle<()> = thread::spawn(move || {
        virtuoso_logger.run();
    });

    #[cfg(feature = "console_backend")]
    let console_backend_thread: thread::JoinHandle<()> = if context.hw_config.is_main_device() {
        thread::spawn(move || {
            console_backend.run();
        })
    } else {
        thread::spawn(|| {})
    };

    #[cfg(feature = "console_backend")]
    if context.hw_config.is_main_device() {
        context.logger.info("Console backend started".to_string());
    }

    #[cfg(feature = "legacy_backend")]
    let legacy_backend_thread: thread::JoinHandle<()> = if context.hw_config.is_main_device() {
        thread::spawn(move || {
            legacy_backend.run();
        })
    } else {
        thread::spawn(|| {})
    };
    #[cfg(feature = "legacy_backend")]
    if context.hw_config.is_main_device() {
        context.logger.info("Legacy backend started".to_string());
    }

    #[cfg(feature = "gpio_frontend")]
    let gpio_frontend_thread = thread::spawn(move || {
        gpio_frontend.run();
    });
    #[cfg(feature = "gpio_frontend")]
    context.logger.info("Gpio frontend started".to_string());

    #[cfg(feature = "cyrano_server")]
    let cyrano_server_thread: thread::JoinHandle<()> = if context.hw_config.is_main_device() && let Ok(cyrano_server) = cyrano_server {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            cyrano_server.run();
        });
        context.logger.info("Cyrano server started".to_string());
        thread
    } else {
        context
            .logger
            .critical_error("Cyrano server did not start because it did not exist".to_string());
        thread::spawn(move || {})
    };

    #[cfg(feature = "repeater")]
    let repeater_thread = if let Ok(repeater) = repeater {
        let thread = thread::spawn(move || {
            repeater.run();
        });
        context.logger.info("Repeater started".to_string());
        thread
    } else {
        context
            .logger
            .critical_error("Repeater did not start because it did not exist".to_string());
        thread::spawn(move || {})
    };

    // };
    // #[cfg(feature = "repeater")]

    #[cfg(feature = "sdl_frontend")]
    {
        use crate::match_info::ProgramState;

        context
            .logger
            .info("sdl frontend started in main thread".to_string());
        sdl_frontend.run();
        context
            .logger
            .info("sdl frontend stopped in main thread".to_string());
        context.match_info.lock().unwrap().program_state = ProgramState::Exiting;
    }

    #[cfg(feature = "legacy_backend")]
    if let Err(e) = legacy_backend_thread.join() {
        context.logger.error(format!(
            "Failed to join legacy backend thread, error: {e:?}"
        ));
    } else {
        context.logger.info("Legacy backend stopped".to_string());
    }

    #[cfg(feature = "gpio_frontend")]
    if let Err(e) = gpio_frontend_thread.join() {
        context.logger.error(format!(
            "Failed to join legacy backend thread, error: {e:?}"
        ));
    } else {
        context.logger.info("Legacy backend stopped".to_string());
    }

    #[cfg(feature = "console_backend")]
    if let Err(e) = console_backend_thread.join() {
        context.logger.error(format!(
            "Failed to join console backend thread, error: {e:?}"
        ));
    } else {
        context.logger.info("Console backend stopped".to_string());
    }

    #[cfg(feature = "cyrano_server")]
    if let Err(e) = cyrano_server_thread.join() {
        context
            .logger
            .error(format!("Failed to join cyrano server thread, error: {e:?}"));
    } else {
        context.logger.info("Cyrano server stopped".to_string());
    }

    #[cfg(feature = "repeater")]
    if let Err(e) = repeater_thread.join() {
        context
            .logger
            .error(format!("Failed to join repeater thread, error: {e:?}"));
    } else {
        context.logger.info("Cyrano server stopped".to_string());
    }

    context.logger.info("Exiting program".to_string());

    context.logger.stop_logger();
    if let Err(e) = logger_thread.join() {
        eprintln!("Failed to join logger thread server thread, error: {e:?}");
    }
}
