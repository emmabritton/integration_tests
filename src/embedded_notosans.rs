use noto_sans_mono_bitmap::{get_raster, FontWeight, RasterHeight};
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::SceneUpdateResult::Pop;
use pixels_graphics_lib::prelude::{Color, KeyCode, Scene, SceneUpdateResult, Timing, BLACK, FxHashSet};
use pixels_graphics_lib::MouseData;

use crate::{SceneName, SceneResult};

pub struct EmbeddedSansScene {
    result: SceneUpdateResult<SceneResult, SceneName>,
}

impl EmbeddedSansScene {
    pub fn new() -> Box<Self> {
        Box::new(EmbeddedSansScene {
            result: SceneUpdateResult::Nothing,
        })
    }
}

impl Scene<SceneResult, SceneName> for EmbeddedSansScene {
    fn render(&self, graphics: &mut Graphics, _: &MouseData, _: &FxHashSet<KeyCode>) {
        graphics.clear(BLACK);

        let start = (16, 16);
        for (idx, chr) in "Hello world".chars().enumerate() {
            let start = (start.0 + (18 * idx), start.1);
            let raster = get_raster(chr, FontWeight::Regular, RasterHeight::Size16).unwrap();
            for (y, row) in raster.raster().iter().enumerate() {
                for (x, pixel) in row.iter().enumerate() {
                    let start = (start.0 + x, start.1 + y);
                    graphics.set_pixel(
                        (start.0 + x) as isize,
                        (start.1 + y) as isize,
                        Color::gray(*pixel),
                    );
                }
            }
        }
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
