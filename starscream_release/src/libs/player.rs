use macroquad::prelude::*;

// Структура игрока
#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub pos: Vec2,     // Позиция игрока
    pub rot: f32,      // Поворот в градусах
    pub vel: Vec2,     // Скорость
    pub v3: [Vec2; 3], // Вершины треугольника (без аллокаций в куче)
}

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

impl Player {
    pub fn new(name: &str) -> Self {
        let pos = Vec2::new(screen_width() / 2., screen_height() / 2.);
        Player {
            name: name.to_string(),
            pos,
            rot: 0.,
            vel: Vec2::new(0., 0.),
            v3: [pos; 3],
        }
    }

    pub fn update(&mut self) {
        let rotation = self.rot.to_radians();
        let mut acc = -self.vel / 100.;

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            acc = Vec2::new(rotation.sin(), -rotation.cos()) / 3.;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.rot += 5.;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rot -= 5.;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            acc = -self.vel / 20.;
        }

        self.vel += acc;
        if self.vel.length() > 5. {
            self.vel = self.vel.normalize() * 5.;
        }
        self.pos += self.vel;

        // Коллизия с границами экрана
        let margin = SHIP_HEIGHT / 2.;
        if self.pos.x < margin {
            self.pos.x = margin;
            self.vel.x = 0.;
        } else if self.pos.x > screen_width() - margin {
            self.pos.x = screen_width() - margin;
            self.vel.x = 0.;
        }

        if self.pos.y < margin {
            self.pos.y = margin;
            self.vel.y = 0.;
        } else if self.pos.y > screen_height() - margin {
            self.pos.y = screen_height() - margin;
            self.vel.y = 0.;
        }

        // Вычисляем вершины корабля
        let v1 = Vec2::new(
            self.pos.x + rotation.sin() * SHIP_HEIGHT / 2.,
            self.pos.y - rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v2 = Vec2::new(
            self.pos.x - rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.pos.y - rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );
        let v3 = Vec2::new(
            self.pos.x + rotation.cos() * SHIP_BASE / 2. - rotation.sin() * SHIP_HEIGHT / 2.,
            self.pos.y + rotation.sin() * SHIP_BASE / 2. + rotation.cos() * SHIP_HEIGHT / 2.,
        );

        self.v3 = [v1, v2, v3];
    }

    /// Только отрисовка (вызываем в конце кадра чтобы корабль был поверх кнопок и текста)
    pub fn draw(&self) {
        draw_triangle_lines(self.v3[0], self.v3[1], self.v3[2], 2., BLACK);
    }
}
