use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::Drawable;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{Point, RgbColor, Size};
use embedded_graphics::text::Text;
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::MouseData;
use pixels_graphics_lib::prelude::{BLACK, KeyCode, Scene, SceneUpdateResult, Timing};
use pixels_graphics_lib::prelude::SceneUpdateResult::Pop;

use crate::{SceneName, SceneResult};

pub struct EmbeddedSegmentsScene {
    result: SceneUpdateResult<SceneResult, SceneName>
}

impl EmbeddedSegmentsScene {
    pub fn new() -> Box<Self> {
        Box::new(EmbeddedSegmentsScene {
            result: SceneUpdateResult::Nothing,
        })
    }
}

impl Scene<SceneResult, SceneName> for EmbeddedSegmentsScene {
    fn render(&self, graphics: &mut Graphics, _: &MouseData, _: &[KeyCode]) {
        graphics.clear(BLACK);

        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(10, 20))
            .digit_spacing(5)
            .segment_width(2)
            .segment_color(Rgb888::GREEN)
            .build();

        Text::new("12:42", Point::new(5, 25), style).draw(graphics).unwrap();

    }

    fn on_key_up(&mut self, key: KeyCode, _: &MouseData, _: &[KeyCode]) {
        if key == KeyCode::Escape {
            self.result = Pop(None);
        }
    }

    fn update(&mut self, _: &Timing, _: &MouseData, _: &[KeyCode]) -> SceneUpdateResult<SceneResult, SceneName> {
        self.result.clone()
    }
}