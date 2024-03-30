use anyhow::Result;
use pixels_graphics_lib::prelude::*;
use crate::embedded_notosans::EmbeddedSansScene;
use crate::embedded_segments::EmbeddedSegmentsScene;

use crate::embedded_u8g2::EmbeddedU8G2Scene;
use crate::menu::MenuScene;

mod embedded_u8g2;
mod menu;
mod embedded_segments;
mod embedded_notosans;

fn main() -> Result<()> {
    let window_prefs = WindowPreferences::new("com", "emmabritton", "int_tester", 1)?;
    let options = Options::default();
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, stack, new_scene| {
        match new_scene {
            SceneName::EmbeddedU8g2 => stack.push(EmbeddedU8G2Scene::new()),
            SceneName::EmbeddedSevenSegment => stack.push(EmbeddedSegmentsScene::new()),
            SceneName::NotoSans => stack.push(EmbeddedSansScene::new()),
        }
    };
    let first_scene = MenuScene::new(&options.style);
    run_scenes(
        300,
        250,
        "Integration Tester",
        Some(window_prefs),
        switcher,
        first_scene,
        options,
        empty_pre_post(),
    )?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum SceneName {
    EmbeddedU8g2,
    EmbeddedSevenSegment,
    NotoSans,
}

#[derive(Clone, Debug, PartialEq)]
enum SceneResult {}
