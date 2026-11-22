use macroquad::miniquad::CursorIcon::Default;

struct Console {
    log: Vec<Vec<String>>,
}

enum Status {
    Error,
    Info,
    Warning,
    Critic,
}

impl Console {
    pub fn new() -> Self {
        Self { log: vec![vec![]] }
    }

    pub fn add(&mut self, st: Status, text: &str) {
        let mut status;
        match st {
            Status::Critic => status = "Critic".to_string(),
            Status::Error => status = "Error".to_string(),
            Status::Info => status = "Info".to_string(),
            Status::Warning => status = "Warn".to_string(),
        }
        status = "Info".to_string();
        self.log.push(vec![status, text.to_string()]);
    }
}
