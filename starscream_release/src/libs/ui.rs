use crate::libs::player::Player;
use macroquad::prelude::*;

// Структура кнопки
pub struct Button {
    rect: Rect,
    text: String,
    text_pos: Vec2,
    font_size: u16,
    font_scale: f32,
    font: Option<Font>,
    progress: f32,
}

// Реализация
impl Button {
    pub fn new(
        corner_left_up: Vec2,
        corner_right_down: Vec2,
        text: &str,
        font: Option<Font>,
    ) -> Self {
        let btn_width = corner_right_down.x - corner_left_up.x;
        let btn_height = corner_right_down.y - corner_left_up.y;
        let rect = Rect::new(corner_left_up.x, corner_left_up.y, btn_width, btn_height);

        let max_text_width = btn_width * 0.85;
        let max_text_height = btn_height * 0.75;

        let base_font_size = 32;
        let base_mes = measure_text(text, font.as_ref(), base_font_size, 1.0);

        let font_scale = if base_mes.width > 0.0 && base_mes.height > 0.0 {
            let scale_x = max_text_width / base_mes.width;
            let scale_y = max_text_height / base_mes.height;
            scale_x.min(scale_y)
        } else {
            1.0
        };

        let mes = measure_text(text, font.as_ref(), base_font_size, font_scale);
        let text_x = rect.center().x - mes.width / 2.0;
        let text_y = rect.center().y + mes.offset_y - mes.height / 2.0;

        Self {
            rect,
            text: text.to_string(),
            text_pos: vec2(text_x, text_y),
            font_size: base_font_size,
            font_scale,
            font,
            progress: 0.0,
        }
    }

    // Возвращает true в случае если игрок стоит долго на кнопке
    pub fn show(&mut self, player: &Player) -> bool {
        let is_inside = self.rect.contains(player.pos);
        let dt = get_frame_time();

        // Заполняем медленнее возвращаем быстрее
        if is_inside {
            self.progress = (self.progress + dt * 1.0).min(1.0);
        } else {
            self.progress = (self.progress - dt * 1.5).max(0.0);
        }

        if self.progress > 0.0 {
            let center = self.rect.center();
            let cur_w = self.rect.w * self.progress;
            let cur_h = self.rect.h * self.progress;

            draw_rectangle(
                center.x - cur_w / 2.0,
                center.y - cur_h / 2.0,
                cur_w,
                cur_h,
                Color::new(1.0, 1.0, 0.85, 0.4 + 0.6 * self.progress),
            );
        }

        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            2.0,
            BLACK,
        );

        let text_color = if self.progress >= 1.0 { GREEN } else { BLUE };
        draw_text_ex(
            &self.text,
            self.text_pos.x,
            self.text_pos.y,
            TextParams {
                font: self.font.as_ref(),
                font_size: self.font_size,
                font_scale: self.font_scale,
                color: text_color,
                ..Default::default()
            },
        );

        // Срабатывает, когда кнопка полностью зарядилась светом
        self.progress >= 1.0
    }
}
