#[allow(dead_code)]
#[allow(unused_variables)]
use std::sync::{Arc, Mutex};
use std::thread;

use match_info::MatchInfo;

mod modules;
mod virtuoso_config;

mod console_backend;
mod match_info;

use crate::modules::VirtuosoModule;
use crate::virtuoso_config::VirtuosoConfig;

#[cfg(feature = "cyrano_server")]
mod cyrano_server;

#[cfg(feature = "legacy_backend")]
mod gpio;
#[cfg(feature = "legacy_backend")]
mod legacy_backend;

#[cfg(feature = "slint_frontend")]
mod layouts;
#[cfg(feature = "slint_frontend")]
mod slint_frontend;

fn main() {
    #[cfg(feature = "video_recorder")]
    todo!();

    let match_info: Arc<Mutex<MatchInfo>> = Arc::new(Mutex::new(MatchInfo::new()));
    let config: Arc<Mutex<VirtuosoConfig>> =
        Arc::new(Mutex::new(VirtuosoConfig::load_config(None)));

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

    #[cfg(feature = "legacy_backend")]
    let legacy_backend_thread = thread::spawn(move || {
        legacy_backend.run();
    });

    #[cfg(feature = "cyrano_server")]
    let cyrano_server_thread = thread::spawn(move || {
        cyrano_server.run();
    });

    #[cfg(feature = "slint_frontend")]
    #[cfg(target_os = "macos")]
    slint_frontend.run();

    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    let slint_frontend_thread = thread::spawn(move || {
        slint_frontend.run();
    });

    #[cfg(feature = "legacy_backend")]
    legacy_backend_thread.join().unwrap();

    #[cfg(feature = "slint_frontend")]
    #[cfg(not(target_os = "macos"))]
    slint_frontend_thread.join().unwrap();

    #[cfg(feature = "console_backend")]
    console_backend_thread.join().unwrap();

    #[cfg(feature = "cyrano_server")]
    cyrano_server_thread.join().unwrap();
}
