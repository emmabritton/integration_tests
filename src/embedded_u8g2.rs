use crate::{SceneName, SceneResult};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::Point;
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::SceneUpdateResult::Pop;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::MouseData;
use u8g2_fonts::fonts::u8g2_font_lubI14_tf;
use u8g2_fonts::types::{FontColor, HorizontalAlignment, VerticalPosition};
use u8g2_fonts::FontRenderer;

pub struct EmbeddedU8G2Scene {
    result: SceneUpdateResult<SceneResult, SceneName>,
    font: FontRenderer,
}

impl EmbeddedU8G2Scene {
    pub fn new() -> Box<Self> {
        Box::new(EmbeddedU8G2Scene {
            result: SceneUpdateResult::Nothing,
            font: FontRenderer::new::<u8g2_font_lubI14_tf>(),
        })
    }
}

impl Scene<SceneResult, SceneName> for EmbeddedU8G2Scene {
    fn render(&self, graphics: &mut Graphics, _: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLACK);

        self.font
            .render_aligned(
                "The quick brown fox jumped\nover the lazy dogs.",
                Point::new(16, 16),
                VerticalPosition::Top,
                HorizontalAlignment::Left,
                FontColor::Transparent(Rgb888::new(255, 255, 255)),
                graphics,
            )
            .unwrap();
    }

    fn on_key_up(&mut self, key: KeyCode, _: &MouseData, _: &FxHashSet<KeyCode>) {
        if key == KeyCode::Escape {
            self.result = Pop(None);
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
}
