use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::match_info::MatchInfo;
use crate::sdl_frontend::colors::*;
use crate::sdl_frontend::widgets::{Label, LabelTextureCache};
use crate::sdl_frontend::{VirtuosoWidget, WidgetContext};

use pnet::datalink;
use pnet::ipnetwork::IpNetwork;

pub struct Drawer<'a> {
    status_widget: Label<'a>,

    texture_cache: LabelTextureCache<'a>,

    last_ip_checked: Instant,

    connected: bool,

    online: bool,
    updated: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Option<Self> {
        if let Some(layout) = &context.layout.cyrano_layout {
            let font: Rc<Font<'_, '_>> = context.get_font(layout.status.font_size);

            Some(Self {
                status_widget: Label::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    font.clone(),
                    layout.status,
                    context.logger,
                ),

                texture_cache: LabelTextureCache::new(),

                last_ip_checked: Instant::now(),
                connected: Self::check_ip(),

                online: false,
                updated: true,
            })
        } else {
            None
        }
    }

    fn check_ip() -> bool {
        let interfaces: Vec<datalink::NetworkInterface> = datalink::interfaces();
        // let interface: Option<&datalink::NetworkInterface> =
        //     interfaces.iter().find(|item| item.name == "end0");
        for interface in interfaces {
            let ipv4: Option<&IpNetwork> = interface
                .ips
                .iter()
                .find(|item: &&IpNetwork| item.is_ipv4());
            if let Some(_ipv4) = ipv4 {
                return true;
            };
        }

        return false;
    }
}

impl<'a> VirtuosoWidget for Drawer<'a> {
    fn update(&mut self, data: &MatchInfo) {
        let connected = if self.last_ip_checked.elapsed() < Duration::from_secs(2) {
            self.connected
        } else {
            self.last_ip_checked = Instant::now();
            Self::check_ip()
        };

        if self.online != data.cyrano_online || self.connected != connected {
            self.online = data.cyrano_online;
            self.connected = connected;
            self.updated = true;
        }
    }

    fn render(&mut self) {
        if self.updated {
            let (text, color) = match (self.online, self.connected) {
                (_, false) => ("OFF".into(), WEAPON_TEXT_DARK),
                (true, true) => ("LAN".into(), CYRANO_ONLINE),
                (false, true) => ("LAN".into(), WEAPON_TEXT_LIGHT),
            };

            self.status_widget
                .render(text, color, Some(&mut self.texture_cache));

            self.updated = false;
        }

        self.status_widget.draw();
    }
}
