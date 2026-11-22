use crate::Scene;
use crate::libs::effects::*;
use crate::libs::network::{self, *};
use crate::libs::player::Player;
use crate::libs::ui::*;
use crate::libs::utils::Timer;
use macroquad::prelude::*;

// Рисуем границы (коллизия границ соблюдается в заголовочном файле [player])
pub fn grand() {
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

// Проверка подключения к основному серверу
pub fn connecting(conn: &mut network::Client, last_reconnect_time: &mut f64, player: &Player) {
    if conn.status == false {
        draw_text(
            "No Connect",
            screen_width() - 115.0,
            screen_height() - 20.0,
            22.0,
            RED,
        );

        let current_time = get_time();
        if current_time - *last_reconnect_time > 2.0 {
            *conn = Client::new("127.0.0.1", "8080");
            *last_reconnect_time = current_time;
        }
    } else {
        // В случае если подключены к серверу то отправляем сразу же данные о своем местоположении имени и других визуализаций
        draw_text(
            "Connect",
            screen_width() - 100.0,
            screen_height() - 20.0,
            22.0,
            GREEN,
        );

        conn.send(player.name.clone(), player.v3);
        conn.show();
    }
}

// Меню игры
pub async fn menu() -> Scene {
    let mut effects = EffectManager::new(); // Эффект взрыва
    let mut title = ParticleText::new("S.T.A.R  S.C.R.E.A.M", vec2(10.0, 100.0), 100, BLACK, None); // Надпись заголовка сверху
    let mut curtain = ScreenDissolve::new(10.0); // Взрыв по частицам
    let mut timer = Timer::new(1.0); // Таймер

    let mut conn = Client::new("0.0.0.0", "8080"); // Подключение к серверу (не блокирует основной поток)
    let mut player = Player::new("Roman"); // Создание игрока и его имя
    let mut last_reconnect_time = 0.0; // Время переподключения к серверу

    // Кнопка играть
    let mut bt_play = Button::new(
        Vec2 { x: 20.0, y: 200.0 },
        Vec2 { x: 450.0, y: 350.0 },
        "Play",
        None,
    );
    // Кнопка настройки
    let mut bt_option = Button::new(
        Vec2 { x: 20.0, y: 400.0 },
        Vec2 { x: 450.0, y: 550.0 },
        "Option",
        None,
    );
    // Кнопка выхода
    let mut bt_exit = Button::new(
        Vec2 { x: 20.0, y: 600.0 },
        Vec2 { x: 450.0, y: 750.0 },
        "Exit",
        None,
    );

    loop {
        clear_background(LIGHTGRAY); // Грейвим экран
        grand(); // Рисуем границы

        if is_key_pressed(KeyCode::Space) {
            effects.spawn_explosion(player.pos, 150.0);
        }
        effects.update_and_draw();

        bt_play.show(&player); // Показываем кнопку play
        bt_option.show(&player); // Показываем кнопку options
        if bt_exit.show(&player) {
            // Показываем кнопку exit
            // Обработка кнопки Exit
            break; // Выходим из игры
        };
        player.update(); // Обновляем кординаты игрока и его Rel
        player.draw(); // Рисуем игрока
        connecting(&mut conn, &mut last_reconnect_time, &player); // Пингуем каждый кадр сервер

        curtain.update_and_draw(); // Анимация взрыва экрана
        if timer.tick(get_frame_time()) {
            // Проверка таймера
            title.update_and_draw(); // В случае если таймер вышел то рисуем текст
        }
        next_frame().await // Следующий кадр
    }
    return Scene::Exit; // Возвращаем в Main перечисление для выхода из игры
}
