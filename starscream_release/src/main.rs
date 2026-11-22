use macroquad::prelude::*;

mod scene {
    pub mod menu;
}

mod libs {
    pub mod network;
    pub mod player;
    pub mod ui;
}

fn window_conf() -> Conf {
    Conf {
        window_title: "".to_owned(),
        window_width: 1000,
        window_height: 1000,
        fullscreen: false,
        ..Default::default()
    }
}

enum Scene {
    Menu,
    Exit,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut sc = Scene::Menu;

    loop {
        match sc {
            Scene::Menu => {
                sc = scene::menu::menu().await;
            }
            Scene::Exit => {
                break;
            }
        }
    }
}
