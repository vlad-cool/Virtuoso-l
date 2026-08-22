use sdl2;
use sdl2::ttf::Font;
use self_update::cargo_crate_version;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time::Duration;

use crate::sdl_frontend::WidgetContext;
use crate::sdl_frontend::colors;
use crate::sdl_frontend::colors::*;
use crate::sdl_frontend::layout_structure::{RectangleProperties, TextProperties};
use crate::sdl_frontend::widgets::{Card, Label};
use crate::settings_menu::MenuElement;
use crate::settings_menu::{MenuItem, SettingsMenu};

pub struct Drawer<'a> {
    status_widget: Label<'a>,
    status: Arc<Mutex<String>>,
    status_string: String,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let status_font: Rc<Font<'_, '_>> = context.get_font(48);

        Self {
            status_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                status_font,
                TextProperties {
                    x: context.layout.background.x,
                    y: context.layout.background.y,
                    width: context.layout.background.width,
                    height: context.layout.background.height,
                    font_size: 48,
                },
                context.logger,
            ),
            status: Arc::new(Mutex::new("".to_string())),
            status_string: "".to_string(),
        }
    }

    pub fn update(&mut self, _data: &SettingsMenu) {}

    pub fn render(&mut self) {
        let new_status: std::sync::MutexGuard<'_, String> = self.status.lock().unwrap();
        if new_status.as_str() != self.status_string {
            self.status_string = new_status.clone();
            self.status_widget
                .render(new_status.clone(), colors::WEAPON_TEXT_LIGHT, None)
        }

        self.status_widget.draw();
    }

    pub fn start_update(&self, state: Arc<AtomicBool>) {
        let status_clone: Arc<Mutex<String>> = self.status.clone();
        thread::spawn(move || Self::update_process(status_clone, state));
    }

    fn update_process(status: Arc<Mutex<String>>, state: Arc<AtomicBool>) {
        let mut backend: self_update::backends::github::UpdateBuilder =
            self_update::backends::github::Update::configure();

        let update_builder: &mut self_update::backends::github::UpdateBuilder = backend
            .repo_owner("vlad-cool")
            .repo_name("Virtuoso-l")
            .bin_name("Virtuoso")
            .no_confirm(true)
            .show_download_progress(false)
            .current_version(cargo_crate_version!());

        let update = match update_builder.build() {
            Ok(update) => update,
            Err(err) => {
                // logger.error(format!("Failed to build updater, err: {err}"));
                *status.lock().unwrap() = format!("Error: {err}");
                std::thread::sleep(Duration::from_secs(10));
                state.store(false, std::sync::atomic::Ordering::Relaxed);
                return;
            }
        };

        let update_status: self_update::Status = match update.update() {
            Ok(status) => status,
            Err(err) => {
                // logger.error(format!("Failed to update, err: {err}"));
                *status.lock().unwrap() = format!("Error: {err}");
                std::thread::sleep(Duration::from_secs(10));
                state.store(false, std::sync::atomic::Ordering::Relaxed);
                return;
            }
        };

        if update_status.updated() {
            *status.lock().unwrap() =
                format!("Successfully\nupdated to\n{}", update_status.version());
        } else {
            *status.lock().unwrap() = format!("Already up to date");
        }
        std::thread::sleep(Duration::from_secs(10));
        state.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}
