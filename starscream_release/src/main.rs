/// REL MACROQUAD PRELUDE
use macroquad::prelude::*;

// Импорт сцен
mod scene {
    pub mod menu;
}

// Импорт всех прикладных библиотек
mod libs {
    pub mod console; // Консоль для отображения информации
    pub mod effects; // Эффекты
    pub mod network; // Работа с сетью
    pub mod player; // Работа с игроком
    pub mod ui; // Виджеты
    pub mod utils; // Утилиты
}

// Перечисление сцен в игре
enum Scene {
    Menu,
    Exit,
}

// Конфигурация окна
fn window_conf() -> Conf {
    Conf {
        window_title: "".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

// Основной поток приложения [.await] [async]
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
