use macroquad::prelude::*;

pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    color: Color,
}

pub struct Explosion {
    center: Vec2,
    max_radius: f32,
    particles: Vec<Particle>,
    timer: f32,
    duration: f32,
    finished: bool,
}

impl Explosion {
    fn new(pos: Vec2, radius: f32) -> Self {
        let count = (radius * 0.35).clamp(20.0, 100.0) as usize;
        let mut particles = Vec::with_capacity(count);
        let colors = [WHITE, YELLOW, ORANGE, RED, DARKGRAY];

        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(radius * 0.5, radius * 2.2);

            particles.push(Particle {
                pos,
                vel: vec2(angle.cos() * speed, angle.sin() * speed),
                size: rand::gen_range(2.0, (radius * 0.04).max(3.0)),
                color: colors[rand::gen_range(0, colors.len())],
            });
        }

        Self {
            center: pos,
            max_radius: radius,
            particles,
            timer: 0.0,
            duration: 0.65,
            finished: false,
        }
    }

    fn update_and_draw(&mut self, dt: f32) {
        if self.finished {
            return;
        }

        self.timer += dt;
        let t = (self.timer / self.duration).min(1.0);

        let (cur_radius, alpha) = if t < 0.2 {
            (self.max_radius * (t / 0.2), 1.0)
        } else {
            let k = (t - 0.2) / 0.8;
            (self.max_radius, (1.0 - k).powi(2))
        };

        draw_circle(
            self.center.x,
            self.center.y,
            cur_radius,
            Color::new(1.0, 0.9, 0.7, alpha * 0.85),
        );

        if t < 0.6 {
            let inner_alpha = (1.0 - t / 0.6) * 0.75;
            draw_circle(
                self.center.x,
                self.center.y,
                cur_radius * 0.6,
                Color::new(1.0, 0.45, 0.1, inner_alpha),
            );
        }

        for p in &mut self.particles {
            p.pos += p.vel * dt;
            p.vel *= 0.92;

            let mut c = p.color;
            c.a = (1.0 - t).max(0.0);
            draw_circle(p.pos.x, p.pos.y, p.size * (1.0 - t * 0.5), c);
        }

        if t >= 1.0 {
            self.finished = true;
        }
    }
}

// Эффект взрыва
pub struct EffectManager {
    explosions: Vec<Explosion>,
}

impl EffectManager {
    pub fn new() -> Self {
        Self {
            explosions: Vec::new(),
        }
    }

    pub fn spawn_explosion(&mut self, pos: Vec2, radius: f32) {
        self.explosions.push(Explosion::new(pos, radius));
    }

    pub fn update_and_draw(&mut self) {
        let dt = get_frame_time();
        for exp in &mut self.explosions {
            exp.update_and_draw(dt);
        }
        self.explosions.retain(|exp| !exp.finished);
    }
}

// Сбор текста по частям
struct TextParticle {
    pos: Vec2,    // Текущая позиция частицы
    target: Vec2, // Итоговая позиция пикселя в букве
    color: Color,
    size: f32,
}

pub struct ParticleText {
    particles: Vec<TextParticle>,
    pub assembled: bool,
}

impl ParticleText {
    pub fn new(text: &str, pos: Vec2, font_size: u16, color: Color, font: Option<&Font>) -> Self {
        let mes = measure_text(text, font, font_size, 1.0);
        let w = (mes.width.ceil() as u32 + 10).max(1);
        let h = (mes.height.ceil() as u32 + 16).max(1);
        let target = render_target(w, h);
        target.texture.set_filter(FilterMode::Nearest);

        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, w as f32, h as f32));
        cam.render_target = Some(target.clone());

        set_camera(&cam);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
        draw_text_ex(
            text,
            5.0,
            mes.offset_y + 5.0,
            TextParams {
                font,
                font_size,
                color,
                ..Default::default()
            },
        );
        set_default_camera();

        let image = target.texture.get_texture_data();
        let mut particles = Vec::new();

        let step = 2;
        for y in (0..h).step_by(step) {
            for x in (0..w).step_by(step) {
                let pixel = image.get_pixel(x, y);
                if pixel.a > 0.2 {
                    let target_pos = vec2(pos.x + x as f32, pos.y + (h - 1 - y) as f32);
                    let start_pos = vec2(
                        rand::gen_range(0.0, screen_width()),
                        rand::gen_range(0.0, screen_height()),
                    );

                    particles.push(TextParticle {
                        pos: start_pos,
                        target: target_pos,
                        color: pixel,
                        size: step as f32,
                    });
                }
            }
        }

        Self {
            particles,
            assembled: false,
        }
    }

    pub fn update_and_draw(&mut self) {
        let dt = get_frame_time();
        let speed = (dt * 4.5).min(1.0);
        let mut all_done = true;

        for p in &mut self.particles {
            let diff = p.target - p.pos;
            if diff.length_squared() > 0.25 {
                p.pos += diff * speed;
                all_done = false;
            } else {
                p.pos = p.target;
            }

            draw_rectangle(p.pos.x, p.pos.y, p.size, p.size, p.color);
        }

        self.assembled = all_done;
    }

    pub fn scatter(&mut self) {
        self.assembled = false;
        for p in &mut self.particles {
            p.pos = vec2(
                rand::gen_range(0.0, screen_width()),
                rand::gen_range(0.0, screen_height()),
            );
        }
    }
}

// Взрыв частиц на весь экран
struct ScreenTile {
    pos: Vec2,
    vel: Vec2,
    size: f32,
    delay: f32,
    alpha: f32,
}

pub struct ScreenDissolve {
    tiles: Vec<ScreenTile>,
    timer: f32,
    pub finished: bool,
}

impl ScreenDissolve {
    pub fn new(tile_size: f32) -> Self {
        let w = screen_width();
        let h = screen_height();
        let center = vec2(w / 2.0, h / 2.0);
        let max_dist = center.length().max(1.0);

        let cols = (w / tile_size).ceil() as usize;
        let rows = (h / tile_size).ceil() as usize;
        let mut tiles = Vec::with_capacity(cols * rows);

        for r in 0..rows {
            for c in 0..cols {
                let pos = vec2(c as f32 * tile_size, r as f32 * tile_size);
                let tile_center = pos + vec2(tile_size / 2.0, tile_size / 2.0);

                let from_center = tile_center - center;
                let dist = from_center.length();
                let dir = if dist > 0.001 {
                    from_center / dist
                } else {
                    vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)).normalize_or_zero()
                };

                let delay = (dist / max_dist) * 0.45 + rand::gen_range(0.0, 0.15);

                let random_spread = vec2(rand::gen_range(-0.4, 0.4), rand::gen_range(-0.4, 0.4));
                let speed = rand::gen_range(250.0, 650.0);

                tiles.push(ScreenTile {
                    pos,
                    vel: (dir + random_spread).normalize_or_zero() * speed,
                    size: tile_size,
                    delay,
                    alpha: 1.0,
                });
            }
        }

        Self {
            tiles,
            timer: 0.0,
            finished: false,
        }
    }

    pub fn update_and_draw(&mut self) {
        if self.finished {
            return;
        }

        let dt = get_frame_time();
        self.timer += dt;
        let mut any_visible = false;

        for tile in &mut self.tiles {
            if self.timer > tile.delay {
                tile.pos += tile.vel * dt;
                tile.vel *= 0.96;
                tile.alpha = (tile.alpha - dt * 1.4).max(0.0);
                tile.size = (tile.size - dt * 10.0).max(0.0);
            }

            if tile.alpha > 0.0 && tile.size > 0.0 {
                any_visible = true;
                draw_rectangle(
                    tile.pos.x,
                    tile.pos.y,
                    tile.size,
                    tile.size,
                    Color::new(0.0, 0.0, 0.0, tile.alpha),
                );
            }
        }

        if !any_visible {
            self.finished = true;
        }
    }
}
