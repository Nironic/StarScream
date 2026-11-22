// Структура таймера
pub struct Timer {
    elapsed: f32,
    duration: f32,
}

// Реализация
impl Timer {
    pub fn new(duration_secs: f32) -> Self {
        Self {
            elapsed: 0.0,
            duration: duration_secs,
        }
    }

    // Обновляет таймер. Возвращает true когда время вышло
    pub fn tick(&mut self, dt: f32) -> bool {
        self.elapsed += dt;
        if self.elapsed >= self.duration {
            //self.elapsed = 0.0; // Автосброс
            // Если нет автосброса то таймер будет бесконечным! Сбрасывать придется самому
            return true;
        }
        false
    }
}
