use sdl2::render::Canvas;
use sdl2::rwops::RWops;
use sdl2::video::{Window, WindowContext};

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

use crate::match_info::MatchInfo;
use crate::sdl_frontend::layout_structure::Layout;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};
use crate::{
    hw_config::{HardwareConfig, Resolution},
    modules::VirtuosoModule,
};

mod colors;
mod layout_structure;
mod layouts;
mod widgets;

#[path = "main_screen/auto_status.rs"]
mod auto_status;
#[path = "main_screen/led_repeater.rs"]
mod led_repeater;
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
    match_info: Arc<Mutex<MatchInfo>>,
    logger: Logger,
    layout: layout_structure::Layout,
}

impl SdlFrontend {
    pub fn new(
        match_info: Arc<Mutex<MatchInfo>>,
        hw_config: HardwareConfig,
        logger: Logger,
    ) -> Self {
        let layout: layout_structure::Layout = match hw_config.display.resolution {
            Resolution::Res1920X1080 => layouts::LAYOUT_1920X1080,
            Resolution::Res1920X550 => layouts::LAYOUT_1920X550,
            Resolution::Res1920X480 => layouts::LAYOUT_1920X480,
            Resolution::Res1920X360 => layouts::LAYOUT_1920X360,
        };

        Self {
            match_info,
            logger,
            layout,
        }
    }
}

impl VirtuosoModule for SdlFrontend {
    fn run(self) {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap_with_logger(&self.logger);
        let video_subsystem: sdl2::VideoSubsystem =
            sdl_context.video().unwrap_with_logger(&self.logger);
        let ttf_context: sdl2::ttf::Sdl2TtfContext = sdl2::ttf::init()
            .map_err(|e| e.to_string())
            .unwrap_with_logger(&self.logger);

        let window: Window = video_subsystem
            .window(
                "Virtuoso",
                self.layout.background.width as u32,
                self.layout.background.height as u32,
            )
            .build()
            .unwrap_with_logger(&self.logger);

        let canvas: Canvas<Window> = window
            .into_canvas()
            .build()
            .unwrap_with_logger(&self.logger);

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
            logger: &self.logger,
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

        canvas.borrow_mut().present();

        let mut modified_count: u32 = 100_000_000;

        let mut event_pump: sdl2::EventPump =
            sdl_context.event_pump().unwrap_with_logger(&self.logger);

        'running: loop {
            std::thread::sleep(Duration::from_millis(20));

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            let data: MutexGuard<'_, MatchInfo> =
                self.match_info.lock().unwrap_with_logger(&self.logger);
            if data.modified_count != modified_count {
                modified_count = data.modified_count;

                for widget in &mut widgets {
                    widget.update(&data);
                }
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
