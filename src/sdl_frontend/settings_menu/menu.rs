use sdl2;
use sdl2::ttf::Font;
use std::rc::Rc;

use crate::sdl_frontend::WidgetContext;
use crate::sdl_frontend::colors::*;
use crate::sdl_frontend::layout_structure::{RectangleProperties, TextProperties};
use crate::sdl_frontend::widgets::{Card, Label};
use crate::settings_menu::MenuElement;
use crate::settings_menu::{MenuItem, SettingsMenu};

pub struct Drawer<'a> {
    header_widget: Label<'a>,
    elements_widgets: [Card<'a>; 5],
}

impl<'a> Drawer<'a> {
    pub fn new(context: WidgetContext<'a>) -> Self {
        let header_font: Rc<Font<'_, '_>> = context.get_font(48);
        let card_font: Rc<Font<'_, '_>> = context.get_font(48);

        Self {
            header_widget: Label::new(
                context.canvas.clone(),
                context.texture_creator,
                header_font.clone(),
                TextProperties {
                    x: 0,
                    y: 0,
                    width: 1920,
                    height: 60,
                    font_size: 48,
                },
                context.logger,
            ),

            elements_widgets: [
                Card::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    card_font.clone(),
                    TextProperties {
                        x: 60 + 300 * 0,
                        y: 60,
                        width: 270,
                        height: 270,
                        font_size: 64,
                    },
                    RectangleProperties {
                        x: 60 + 300 * 0,
                        y: 60,
                        width: 270,
                        height: 270,
                        radius: 10,
                    },
                    context.logger,
                ),
                Card::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    card_font.clone(),
                    TextProperties {
                        x: 60 + 300 * 1,
                        y: 60,
                        width: 270,
                        height: 270,
                        font_size: 64,
                    },
                    RectangleProperties {
                        x: 60 + 300 * 1,
                        y: 60,
                        width: 270,
                        height: 270,
                        radius: 10,
                    },
                    context.logger,
                ),
                Card::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    card_font.clone(),
                    TextProperties {
                        x: 60 + 300 * 2,
                        y: 60,
                        width: 270,
                        height: 270,
                        font_size: 64,
                    },
                    RectangleProperties {
                        x: 60 + 300 * 2,
                        y: 60,
                        width: 270,
                        height: 270,
                        radius: 10,
                    },
                    context.logger,
                ),
                Card::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    card_font.clone(),
                    TextProperties {
                        x: 60 + 300 * 3,
                        y: 60,
                        width: 270,
                        height: 270,
                        font_size: 64,
                    },
                    RectangleProperties {
                        x: 60 + 300 * 3,
                        y: 60,
                        width: 270,
                        height: 270,
                        radius: 10,
                    },
                    context.logger,
                ),
                Card::new(
                    context.canvas.clone(),
                    context.texture_creator,
                    card_font.clone(),
                    TextProperties {
                        x: 60 + 300 * 4,
                        y: 60,
                        width: 270,
                        height: 270,
                        font_size: 64,
                    },
                    RectangleProperties {
                        x: 60 + 300 * 4,
                        y: 60,
                        width: 270,
                        height: 270,
                        radius: 10,
                    },
                    context.logger,
                ),
            ],
        }
    }

    pub fn update(&mut self, data: &SettingsMenu) {
        let elements: Vec<MenuElement> = data.get_item().get_elements();
        let index: usize = data.get_item().get_index();
        // for i in 0..(std::cmp::min(elements.len(), 4)) {
        for i in 0..(self.elements_widgets.len() - 1) {
            if i < elements.len() {
                let element: &MenuElement = &elements[i];

                let selected: bool = index == i;
                match element.to_item() {
                    MenuItem::Label(s) => {
                        self.elements_widgets[i].render(
                            s.as_str(),
                            if selected { 4 } else { 2 },
                            MENU_LABEL_BACKGROUND,
                            MENU_LABEL_FRAME,
                            MENU_LABEL_TEXT,
                        );
                    }
                    MenuItem::Button(s) => {
                        self.elements_widgets[i].render(
                            s.as_str(),
                            if selected { 4 } else { 2 },
                            MENU_BUTTON_BACKGROUND,
                            MENU_BUTTON_FRAME,
                            MENU_BUTTON_TEXT,
                        );
                    }
                }
            } else {
                self.elements_widgets[i].render(
                    " ",
                    0,
                    BACKGROUND,
                    BACKGROUND,
                    BACKGROUND,
                );
            }
        }
        self.header_widget
            .render(data.get_item().get_name(), MENU_HEADER_TEXT, None);
    }

    pub fn render(&mut self) {
        self.header_widget.draw();
        for elememt_widget in &mut self.elements_widgets {
            elememt_widget.draw();
        }
    }
}
