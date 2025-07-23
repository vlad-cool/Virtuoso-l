use sdl2::render::Canvas;
use sdl2::rwops::RWops;
use sdl2::video::{Window, WindowContext};

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

use crate::layout_structure::Layout;
use crate::match_info::MatchInfo;
use crate::match_info::Weapon;
use crate::virtuoso_logger::{Logger, LoggerUnwrap};
use crate::{colors, match_info};
use crate::{
    hw_config::{HardwareConfig, Resolution},
    modules::VirtuosoModule,
};
use crate::{layout_structure, layouts};

mod auto_status;
mod led_repeater;
mod message;
mod passive_card;
mod passive_counter;
mod passive_indicator;
mod penalty_card;
mod period;
mod priority;
mod score;
mod timer;
mod weapon;
mod widgets;

const MESSAGE_DISPLAY_TIME: Duration = Duration::from_secs(2);

#[derive(Clone)]
struct WidgetContext<'a> {
    pub canvas: Rc<RefCell<Canvas<Window>>>,
    pub texture_creator: &'a sdl2::render::TextureCreator<WindowContext>,
    pub ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    pub font_bytes: &'a [u8],
    pub layout: &'a Layout,
    pub logger: &'a Logger,
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

        let font_bytes = include_bytes!("../../assets/AGENCYB.ttf");

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

        widgets.push(Box::new(auto_status::Drawer::new(widget_context)));

        let mut score_drawer: score::Drawer<'_> = score::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut message: message::Drawer<'_> = message::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut weapon_drawer: weapon::Drawer<'_> = weapon::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut period_drawer: period::Drawer<'_> = period::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut timer_drawer: timer::Drawer<'_> = timer::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut passive_counter: passive_counter::Drawer<'_> = passive_counter::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut passive_indicator: passive_indicator::Drawer<'_> = passive_indicator::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &self.layout,
            &self.logger,
        );

        let mut passive_card: passive_card::Drawer<'_> = passive_card::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut penalty_card: penalty_card::Drawer<'_> = penalty_card::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        // let mut auto_status: auto_status::Drawer<'_> = auto_status::Drawer::new(
        //     canvas.clone(),
        //     &texture_creator,
        //     &ttf_context,
        //     RWops::from_bytes(font_bytes)
        //         .map_err(|e| e.to_string())
        //         .unwrap_with_logger(&self.logger),
        //     &self.layout,
        //     &self.logger,
        // );

        let mut priority: priority::Drawer<'_> = priority::Drawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap_with_logger(&self.logger),
            &self.layout,
            &self.logger,
        );

        let mut led_repeater: led_repeater::Drawer<'_> =
            led_repeater::Drawer::new(canvas.clone(), &texture_creator, &self.layout, &self.logger);

        canvas.borrow_mut().present();

        let mut modified_count: u32 = 999;

        let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap_with_logger(&self.logger);
        'running: loop {
            std::thread::sleep(Duration::from_millis(50));

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            let data: MutexGuard<'_, MatchInfo> =
                self.match_info.lock().unwrap_with_logger(&self.logger);

            if data.modified_count == modified_count {
                continue 'running;
            }

            modified_count = data.modified_count;

            canvas.borrow_mut().clear();

            let display_message: bool = data.display_message != ""
                && data.display_message_updated.elapsed() < MESSAGE_DISPLAY_TIME;
            if display_message {
                message.render(data.display_message.clone());
            } else {
                timer_drawer.render(data.timer_controller.get_time(), true, data.priority);
            }
            score_drawer.render(data.left_fencer.score, data.right_fencer.score);
            weapon_drawer.render(data.weapon);
            period_drawer.render(data.period);
            passive_counter.render(
                data.passive_timer.get_counter(),
                data.weapon != Weapon::Fleuret,
            );
            passive_card.render(
                data.left_fencer.passive_card,
                data.right_fencer.passive_card,
            );
            penalty_card.render(
                data.left_fencer.warning_card,
                data.right_fencer.warning_card,
            );
            // auto_status.render(data.auto_timer_on, data.auto_score_on);
            priority.render(data.priority);
            led_repeater.render(
                data.left_fencer.color_light,
                data.left_fencer.white_light,
                data.right_fencer.color_light,
                data.right_fencer.white_light,
            );

            if display_message {
                message.draw();
            } else {
                timer_drawer.draw();
            }
            score_drawer.draw();
            weapon_drawer.draw();
            period_drawer.draw();
            passive_counter.draw();
            passive_card.draw();
            penalty_card.draw();
            // auto_status.draw();
            priority.draw();
            led_repeater.draw();

            for widget in &mut widgets {
                widget.update(&data);
            }
            std::mem::drop(data);
            for widget in &mut widgets {
                widget.render();
            }
            

            canvas.borrow_mut().present();
        }
    }
}
