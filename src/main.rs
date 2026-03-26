use std::thread;
use std::time::Duration;

use macroquad::prelude::*;

use snake::sim::Simulation;
use snake::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "snake-ai".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 1,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sim = Simulation::new();
    let mut is_viz_enabled = true;
    let mut is_nn_enabled = true;
    let mut is_slow_mode = false;
    let mut speed: usize = 50;

    loop {
        let mut iterations = 0;

        loop {
            sim.update();
            iterations += 1;

            if is_slow_mode || iterations >= speed {
                break;
            }
        }

        sim.update_settings(is_viz_enabled, is_nn_enabled, is_slow_mode, speed);
        sim.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            is_viz_enabled = !is_viz_enabled;
        }
        if is_key_pressed(KeyCode::N) {
            is_nn_enabled = !is_nn_enabled;
        }
        if is_key_released(KeyCode::Space) {
            is_slow_mode = !is_slow_mode;
        }
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            speed = (speed + 50).min(1000);
        }
        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            speed = speed.saturating_sub(50).max(1);
        }
        if is_key_pressed(KeyCode::Key1) { speed = 50; }
        if is_key_pressed(KeyCode::Key2) { speed = 100; }
        if is_key_pressed(KeyCode::Key3) { speed = 250; }
        if is_key_pressed(KeyCode::Key4) { speed = 500; }
        if is_key_pressed(KeyCode::Key5) { speed = 1000; }

        if is_slow_mode {
            thread::sleep(Duration::from_millis(SIM_SLEEP_MILLIS));
        }
        next_frame().await
    }
}
