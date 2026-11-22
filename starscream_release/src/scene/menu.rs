use crate::Scene;
use crate::libs::network::{self, *};
use crate::libs::player::Player;
use crate::libs::ui::*;
use macroquad::prelude::*;

/// Границы
pub fn grand() {
    // Границы
    draw_line(0., 1., screen_width() - 1., 1., 1., RED);
    draw_line(
        0.,
        screen_height(),
        screen_width() - 1.,
        screen_height(),
        1.,
        RED,
    );
    draw_line(1., 0., 0., screen_height(), 1., RED);
    draw_line(screen_width(), 0., screen_width(), screen_height(), 1., RED);
}

pub fn connecting(conn: &mut network::Client, last_reconnect_time: &mut f64, player: &Player) {
    // Проблема с коннектом
    if conn.status == false {
        draw_text(
            "No Connect",
            screen_width() - 115.0,
            screen_height() - 20.0,
            22.0,
            RED, // Сделаем ошибку красной для наглядности
        );

        // Пытаемся переподключиться не каждый кадр, а раз в 2 секунды
        let current_time = get_time();
        if current_time - *last_reconnect_time > 2.0 {
            *conn = Client::new("127.0.0.1", "8080");
            *last_reconnect_time = current_time;
        }
    } else {
        draw_text(
            "Connect",
            screen_width() - 100.0,
            screen_height() - 20.0,
            22.0,
            GREEN,
        );

        conn.send(player.name.clone(), &player.v3);
        conn.show();
    }
}

pub async fn menu() -> Scene {
    let mut conn = Client::new("0.0.0.0", "8080");
    let mut player = Player::new("Roman");
    let mut last_reconnect_time = 0.0;
    loop {
        clear_background(LIGHTGRAY);
        grand();

        button(
            Vec2 { x: 100.0, y: 500.0 },
            Vec2 { x: 450.0, y: 650.0 },
            "Play",
            &player,
            None,
        );
        let pos = player.show();

        connecting(&mut conn, &mut last_reconnect_time, &player);

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
    return Scene::Exit;
}
