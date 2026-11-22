use crate::libs::player::Player;
use macroquad::prelude::*;

pub fn button(
    corner_left_up: Vec2,
    corner_right_down: Vec2,
    text: &str,
    player: &Player,
    font: Option<&Font>,
) -> bool {
    let btn_width = corner_right_down.x - corner_left_up.x;
    let btn_height = corner_right_down.y - corner_left_up.y;

    let center_x = corner_left_up.x + btn_width / 2.0;
    let center_y = corner_left_up.y + btn_height / 2.0;

    // 1. Задаем внутренние отступы (например, 15% от размеров кнопки)
    let max_text_width = btn_width * 0.85;
    let max_text_height = btn_height * 0.75;

    // 2. Измеряем текст при масштабе 1.0, чтобы узнать его пропорции
    let base_font_size = 32; // Базовое разрешение шрифта (чем больше, тем четче буквы)
    let base_mes = measure_text(text, font, base_font_size, 1.0);

    // 3. Вычисляем идеальный масштаб под размер кнопки (с защитой от пустой строки)
    let font_scale = if base_mes.width > 0.0 && base_mes.height > 0.0 {
        let scale_x = max_text_width / base_mes.width;
        let scale_y = max_text_height / base_mes.height;
        scale_x.min(scale_y)
    } else {
        1.0
    };

    // 4. Финальный замер с итоговым масштабом для точного центрирования
    let mes = measure_text(text, font, base_font_size, font_scale);

    let text_x = center_x - mes.width / 2.0;
    let text_y = center_y + mes.offset_y - mes.height / 2.0;

    // 5. Рисуем текст через draw_text_ex с теми же font, font_size и font_scale
    draw_text_ex(
        text,
        text_x,
        text_y,
        TextParams {
            font,
            font_size: base_font_size,
            font_scale,
            color: BLACK,
            ..Default::default()
        },
    );

    // 6. Рисуем рамку кнопки (в macroquad есть готовая функция вместо 4 линий)
    draw_rectangle_lines(
        corner_left_up.x,
        corner_left_up.y,
        btn_width,
        btn_height,
        2.0,
        BLACK,
    );

    if player.pos.x > corner_left_up.x
        && player.pos.x < corner_left_up.x + btn_width
        && player.pos.y > corner_left_up.y
        && player.pos.y < corner_left_up.y + btn_height
    {
        return true;
    }

    false
}
