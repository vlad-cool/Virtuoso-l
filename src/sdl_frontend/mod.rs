use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rwops::RWops;

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use std::cell::RefCell;
use std::rc::Rc;

use crate::modules;
use crate::virtuoso_logger::Logger;
use crate::{colors, match_info};
use crate::{
    hw_config::{HardwareConfig, Resolution},
    modules::VirtuosoModule,
};
use crate::{layout_structure, layouts};

mod period;
mod renderers;
mod score;
mod weapon;

const MESSAGE_DISPLAY_TIME: Duration = Duration::from_secs(2);

pub struct SdlFrontend {
    match_info: Arc<Mutex<match_info::MatchInfo>>,
    hw_config: HardwareConfig,
    logger: Logger,
    layout: layout_structure::Layout,
}

impl SdlFrontend {
    pub fn new(
        match_info: Arc<Mutex<match_info::MatchInfo>>,
        hw_config: HardwareConfig,
        logger: Logger,
    ) -> Self {
        let layout = match hw_config.display.resolution {
            Resolution::Res1920X1080 => layouts::LAYOUT_1920X1080,
            Resolution::Res1920X550 => layouts::LAYOUT_1920X550,
            Resolution::Res1920X480 => layouts::LAYOUT_1920X480,
            Resolution::Res1920X360 => layouts::LAYOUT_1920X360,
        };

        Self {
            match_info,
            hw_config,
            logger,
            layout,
        }
    }
}

impl VirtuosoModule for SdlFrontend {
    fn run(mut self) {
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
        let video_subsystem: sdl2::VideoSubsystem = sdl_context.video().unwrap();
        let ttf_context: sdl2::ttf::Sdl2TtfContext =
            sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let window: sdl2::video::Window = video_subsystem
            .window(
                "Virtuoso",
                self.layout.background.width as u32,
                self.layout.background.height as u32,
            )
            .build()
            .unwrap();

        let mut canvas: sdl2::render::Canvas<sdl2::video::Window> =
            window.into_canvas().build().unwrap();

        let canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>> =
            Rc::new(RefCell::new(canvas));

        canvas.borrow_mut().set_draw_color(colors::BACKGROUND);

        let font_bytes = include_bytes!("../../assets/AGENCYB.ttf");

        let texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext> =
            canvas.borrow().texture_creator();

        let mut score_drawer: score::ScoreDrawer<'_> = score::ScoreDrawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap(),
            &self.layout,
            &self.logger,
        );

        let mut weapon_drawer: weapon::WeaponDrawer<'_> = weapon::WeaponDrawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap(),
            &self.layout,
            &self.logger,
        );

        let mut period_drawer: period::PeriodDrawer<'_> = period::PeriodDrawer::new(
            canvas.clone(),
            &texture_creator,
            &ttf_context,
            RWops::from_bytes(font_bytes)
                .map_err(|e| e.to_string())
                .unwrap(),
            &self.layout,
            &self.logger,
        );

        let mut i: u32 = 0;

        let mut event_pump = sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }
            canvas.borrow_mut().clear();
            score_drawer.render(i, i + 2);
            i += 1;
            i %= 55;
            weapon_drawer.render(match_info::Weapon::Fleuret);
            period_drawer.render(7);

            score_drawer.draw();
            weapon_drawer.draw();
            period_drawer.draw();

            canvas.borrow_mut().present();
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}
