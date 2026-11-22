use macroquad::prelude::*;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub pos: Vec2,     // Позиция игрока
    pub rot: f32,      // Поворот в градусах
    pub vel: Vec2,     // Ускорение
    pub v3: Vec<Vec2>, // Вектора ускорения позиций
}

const SHIP_HEIGHT: f32 = 25.;
const SHIP_BASE: f32 = 22.;

impl Player {
    pub fn new(name: &str) -> Self {
        // Создание игрока
        Player {
            name: name.to_string(),
            pos: Vec2::new(screen_width() / 2., screen_height() / 2.), // Спавним плеера в середине экрана
            rot: 0.,                // Поворот на сколько градусов
            vel: Vec2::new(0., 0.), // Ускорение
            v3: vec![Vec2::new(10.0, 10.0)],
        }
    }

    pub fn show(&mut self) {
        // Управление кораблем
        let rotation = self.rot.to_radians();
        let mut acc = -self.vel / 100.;

        // Нажатие клавишь и управление кораблем
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

        // --- НАЧАЛО БЛОКА КОЛЛИЗИИ С ГРАНИЦАМИ ---
        // Берем половину максимального размера корабля как отступ от краев
        let margin = SHIP_HEIGHT / 2.;

        // Ограничиваем координату X по ширине экрана
        if self.pos.x < margin {
            self.pos.x = margin;
            self.vel.x = 0.; // Гасим скорость при ударе о стену
        } else if self.pos.x > screen_width() - margin {
            self.pos.x = screen_width() - margin;
            self.vel.x = 0.;
        }

        // Ограничиваем координату Y по высоте экрана
        if self.pos.y < margin {
            self.pos.y = margin;
            self.vel.y = 0.; // Гасим скорость при ударе о потолок/пол
        } else if self.pos.y > screen_height() - margin {
            self.pos.y = screen_height() - margin;
            self.vel.y = 0.;
        }
        // --- КОНЕЦ БЛОКА КОЛЛИЗИИ С ГРАНИЦАМИ ---

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

        draw_triangle_lines(v1, v2, v3, 2., BLACK);

        self.v3 = vec![v1, v2, v3];
    }
}
