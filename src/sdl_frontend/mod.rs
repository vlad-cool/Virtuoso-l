use sdl2::render::Canvas;
use sdl2::rwops::RWops;
use sdl2::video::{Window, WindowContext};

use std::sync::MutexGuard;
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

use crate::hw_config::Resolution;
use crate::match_info::MatchInfo;
use crate::modules::VirtuosoModule;
use crate::modules::VirtuosoModuleContext;
use crate::sdl_frontend::layout_structure::Layout;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};

mod colors;
mod layout_structure;
mod layouts;
mod widgets;

// #[path = "main_screen/cyrano/cyrano_status.rs"]
// mod cyrano_status;
#[path = "main_screen/cyrano/competition_type.rs"]
mod competition_type;
#[path = "main_screen/cyrano/fencer_id.rs"]
mod fencer_id;
#[path = "main_screen/cyrano/fencer_medical.rs"]
mod fencer_medical;
#[path = "main_screen/cyrano/fencer_name.rs"]
mod fencer_name;
#[path = "main_screen/cyrano/fencer_nation.rs"]
mod fencer_nation;
#[path = "main_screen/cyrano/fencer_reserve.rs"]
mod fencer_reserve;
#[path = "main_screen/cyrano/fencer_status.rs"]
mod fencer_status;
#[path = "main_screen/cyrano/phase.rs"]
mod phase;
#[path = "main_screen/cyrano/piste.rs"]
mod piste;
#[path = "main_screen/cyrano/poul_tab.rs"]
mod poul_tab;
#[path = "main_screen/cyrano/start_time.rs"]
mod start_time;
#[path = "main_screen/cyrano/cyrano_state.rs"]
mod cyrano_state;
#[path = "main_screen/cyrano/referee_name.rs"]
mod referee_name;
#[path = "main_screen/cyrano/referee_nation.rs"]
mod referee_nation;

#[path = "main_screen/auto_status.rs"]
mod auto_status;
#[path = "main_screen/led_repeater.rs"]
mod led_repeater;
#[path = "settings_menu/menu.rs"]
mod menu;
#[path = "main_screen/message.rs"]
mod message;
#[path = "main_screen/passive_card.rs"]
mod passive_card;
#[path = "main_screen/passive_counter.rs"]
mod passive_counter;
#[path = "main_screen/passive_indicator.rs"]
mod passive_indicator;
#[path = "main_screen/penalty_card.rs"]
mod penalty_card;
#[path = "main_screen/period.rs"]
mod period;
#[path = "main_screen/priority.rs"]
mod priority;
#[path = "main_screen/score.rs"]
mod score;
#[path = "main_screen/timer.rs"]
mod timer;
#[path = "main_screen/weapon.rs"]
mod weapon;

#[derive(Clone)]
struct WidgetContext<'a> {
    pub canvas: Rc<RefCell<Canvas<Window>>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<WindowContext>,
    pub ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    pub font_bytes: &'a [u8],
    pub layout: &'a Layout,
    pub logger: &'a Logger,
}

impl<'a> WidgetContext<'a> {
    pub fn get_font(&self, font_size: u16) -> Rc<sdl2::ttf::Font<'a, 'a>> {
        let rwops: RWops<'_> = RWops::from_bytes(self.font_bytes).unwrap_with_logger(self.logger);
        let font: sdl2::ttf::Font<'a, 'a> = self
            .ttf_context
            .load_font_from_rwops(rwops, font_size)
            .unwrap_with_logger(self.logger);
        Rc::new(font)
    }
}

trait VirtuosoWidget {
    fn update(&mut self, data: &MatchInfo);
    fn render(&mut self);
}

pub struct SdlFrontend {
    context: VirtuosoModuleContext,
    layout: layout_structure::Layout,
}

impl SdlFrontend {
    pub fn new(context: VirtuosoModuleContext) -> Self {
        let layout: layout_structure::Layout = match context.hw_config.display.resolution {
            Resolution::Res1920X1080 => layouts::LAYOUT_1920X1080,
            Resolution::Res1920X550 => layouts::LAYOUT_1920X550,
            Resolution::Res1920X480 => layouts::LAYOUT_1920X480,
            Resolution::Res1920X360 => layouts::LAYOUT_1920X360,
        };

        Self { context, layout }
    }
}

impl VirtuosoModule for SdlFrontend {
    fn run(self) {
        let sdl_context: sdl2::Sdl = match sdl2::init() {
            Ok(sdl_context) => sdl_context,
            Err(err) => {
                self.context
                    .logger
                    .critical_error(format!("Failed to init sdl, error: {err}"));
                return;
            }
        };

        let video_subsystem: sdl2::VideoSubsystem = match sdl_context.video() {
            Ok(video_subsystem) => video_subsystem,
            Err(err) => {
                self.context
                    .logger
                    .critical_error(format!("Failed to create video subsystem, error: {err}"));
                return;
            }
        };

        let ttf_context: sdl2::ttf::Sdl2TtfContext = match sdl2::ttf::init() {
            Ok(ttf_context) => ttf_context,
            Err(err) => {
                self.context
                    .logger
                    .critical_error(format!("Failed to init ttf context, error: {err}"));
                return;
            }
        };

        let window: Window = match video_subsystem
            .window(
                "Virtuoso",
                self.layout.background.width as u32,
                self.layout.background.height as u32,
            )
            .build()
        {
            Ok(window) => window,
            Err(err) => {
                self.context
                    .logger
                    .critical_error(format!("Failed to create window, error: {err}"));
                return;
            }
        };

        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(err) => {
                self.context
                    .logger
                    .critical_error(format!("Failed to create canvas, error: {err}"));
                return;
            }
        };

        let canvas: Rc<RefCell<Canvas<Window>>> = Rc::new(RefCell::new(canvas));

        canvas.borrow_mut().set_draw_color(colors::BACKGROUND);

        let font_bytes: &[u8] = include_bytes!("../../assets/AGENCYB.ttf") as &[u8];

        let texture_creator: sdl2::render::TextureCreator<WindowContext> =
            canvas.borrow().texture_creator();

        let widget_context: WidgetContext<'_> = WidgetContext {
            canvas: canvas.clone(),
            texture_creator: &texture_creator,
            ttf_context: &ttf_context,
            font_bytes,
            layout: &self.layout,
            logger: &self.context.logger,
        };

        canvas.borrow_mut().clear();

        let mut widgets: Vec<Box<dyn VirtuosoWidget>> = Vec::<Box<dyn VirtuosoWidget>>::new();

        widgets.push(Box::new(auto_status::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(led_repeater::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(message::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(passive_card::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(passive_counter::Drawer::new(
            widget_context.clone(),
        )));
        widgets.push(Box::new(passive_indicator::Drawer::new(
            widget_context.clone(),
        )));
        widgets.push(Box::new(penalty_card::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(period::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(priority::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(score::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(timer::Drawer::new(widget_context.clone())));
        widgets.push(Box::new(weapon::Drawer::new(widget_context.clone())));

        // widgets.push(Box::new(fencer_name::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(fencer_nation::Drawer::new(widget_context.clone())));
        // widgets.push(Box::new(cyrano_status::Drawer::new(widget_context.clone())));

        if let Some(widget) = referee_name::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = referee_nation::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = fencer_name::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = fencer_id::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = fencer_nation::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = fencer_status::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = fencer_medical::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = fencer_reserve::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = piste::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = phase::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = competition_type::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = poul_tab::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = start_time::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }
        if let Some(widget) = cyrano_state::Drawer::new(widget_context.clone()) {
            widgets.push(Box::new(widget));
        }

        let mut settings_menu: menu::Drawer<'_> = menu::Drawer::new(widget_context.clone());

        canvas.borrow_mut().present();

        let mut event_pump: sdl2::EventPump = sdl_context
            .event_pump()
            .unwrap_with_logger(&self.context.logger);

        'running: loop {
            std::thread::sleep(Duration::from_millis(20));

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            if self
                .context
                .settings_menu_shown
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                canvas.borrow_mut().clear();
                settings_menu.update(&self.context.settings_menu.lock().unwrap());
                settings_menu.render();
                canvas.borrow_mut().present();
                std::thread::sleep(Duration::from_millis(40));
            } else {
                let data: MutexGuard<'_, MatchInfo> = self
                    .context
                    .match_info
                    .lock()
                    .unwrap_with_logger(&self.context.logger);
                for widget in &mut widgets {
                    widget.update(&data);
                }
                std::mem::drop(data);

                canvas.borrow_mut().clear();
                for widget in &mut widgets {
                    widget.render();
                }
                canvas.borrow_mut().present();
            }
        }
    }
}
