use crate::{SceneName, SceneResult};
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::SceneUpdateResult::Nothing;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::scenes::SceneUpdateResult::Push;
use pixels_graphics_lib::ui::prelude::*;

pub struct MenuScene {
    bg_color: Color,
    labels: Vec<Text>,
    result: SceneUpdateResult<SceneResult, SceneName>,
    u8g2: Button,
    segments: Button,
    noto: Button,
}

impl MenuScene {
    pub fn new(style: &UiStyle) -> Box<Self> {
        Box::new(MenuScene {
            bg_color: style.background,
            labels: vec![
                Text::new(
                    "Embedded Graphics",
                    TextPos::px(coord!(16, 16)),
                    style.title_text.clone(),
                ),
                Text::new(
                    "Noto Sans Bitmap",
                    TextPos::px(coord!(16, 100)),
                    style.title_text.clone(),
                ),
            ],
            u8g2: Button::new(coord!(20, 30), "U8G2", Some(100), &style.button),
            segments: Button::new(coord!(20, 52), "Segments", Some(100), &style.button),
            noto: Button::new(coord!(20, 116), "Noto Sans", Some(100), &style.button),
            result: Nothing,
        })
    }
}

impl Scene<SceneResult, SceneName> for MenuScene {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(self.bg_color);
        for text in &self.labels {
            text.render(graphics);
        }
        self.u8g2.render(graphics, mouse);
        self.segments.render(graphics, mouse);
        self.noto.render(graphics, mouse);
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        mouse_button: MouseButton,
        _: &FxHashSet<KeyCode>,
    ) {
        if mouse_button == MouseButton::Left {
            if self.u8g2.on_mouse_click(down_at, mouse.xy) {
                self.result = Push(false, SceneName::EmbeddedU8g2);
            }
            if self.segments.on_mouse_click(down_at, mouse.xy) {
                self.result = Push(false, SceneName::EmbeddedSevenSegment);
            }
            if self.noto.on_mouse_click(down_at, mouse.xy) {
                self.result = Push(false, SceneName::NotoSans);
            }
        }
    }

    fn update(
        &mut self,
        _: &Timing,
        _: &MouseData,
        _: &FxHashSet<KeyCode>,
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        self.result.clone()
    }

    fn resuming(&mut self, _: Option<SceneResult>) {
        self.result = Nothing;
    }
}
