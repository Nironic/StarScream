// Реализация сетевой игры
use macroquad::math::Vec2;
use macroquad::*;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

// PROTOCOL
// Name,     pos[0]        pos[1]         pos[2]
// Roman, [10.0, 50.0], [30.0, 110.0], [99.0, 23.0]
/*
pub struct Player {
    pub name: String,
    pub pos: Vec2, // Позиция игрока
    pub rot: f32,  // Поворот в градусах
    pub vel: Vec2, // Ускорение
}
*/
struct Player {
    name: String,
    pos: Vec<Vec2>,
}

pub struct Client {
    pub sock: Option<TcpStream>,
    pub status: bool,
    pub cl: Vec<Player>,
}

pub struct RetErr {
    pub soul: String,
    pub status: bool,
}

impl Client {
    pub fn new(ip: &str, port: &str) -> Self {
        match TcpStream::connect(format!("{}:{}", ip, port)) {
            Ok(stream) => {
                // Метод set_nonblocking вызывается прямо на TcpStream
                // Если он вернет ошибку, unwrap() вызовет панику (или обработайте через match/if let)
                stream.set_nonblocking(true).unwrap();

                Self {
                    sock: Some(stream),
                    status: true,
                    cl: vec![],
                }
            }
            Err(_err) => {
                // Если подключение не удалось, сразу возвращаем структуру с None
                Self {
                    sock: None,
                    status: false,
                    cl: vec![],
                }
            }
        }
    }

    pub fn send(&mut self, name: String, pos: &Vec<Vec2>) -> bool {
        // Форматируем строку (обязательно с \n)
        let str_form = format!(
            "{}:{}:{}:{}:{}:{}:{}\n",
            name,
            pos[0][0] as f32,
            pos[0][1] as f32,
            pos[1][0] as f32,
            pos[1][1] as f32,
            pos[2][0] as f32,
            pos[2][1] as f32
        );

        // .as_mut() превращает &mut Option<TcpStream> в Option<&mut TcpStream>
        // Теперь мы работаем по ссылке, не забирая сокет из структуры
        if let Some(ref mut sock) = self.sock {
            // Импортируйте std::io::Write в начале файла, чтобы методы ниже работали
            if sock.write_all(str_form.as_bytes()).is_err() {
                self.status = false;
                return false;
            }
            if sock.flush().is_err() {
                self.status = false;
                return false;
            }
            true
        } else {
            // Сокета вообще нет (status == false)
            self.status = false;
            false
        }
    }

    pub fn show(&mut self) -> bool {
        // Ждем такой формат протокола
        /*
        let str_form = format!(
            "{}:{}:{}:{}:{}:{}:{}\n",
            name, pos[0][0], pos[0][1], pos[1][0], pos[1][1], pos[2][0], pos[2][1]
        );

        RESULT: Name,10.0,20.0,30.0,40.0,50.0,60.0
        */
        let result = self.recv();
        if result.status == false {
            return false;
        }

        if result.soul.len() == 0 {
            return true;
        }

        // Форматируемся
        {
            let format_str: Vec<&str> = result.soul.split(":").collect();

            if format_str.len() < 7 {
                println!("Предупреждение: неверный формат пакета от сервера");
                return false;
            }
            let name = format_str[0]; // &str
            let v1_1: f32 = format_str[1].parse().unwrap_or(0.0);
            let v1_2: f32 = format_str[2].parse().unwrap_or(0.0);
            let v2_1: f32 = format_str[3].parse().unwrap_or(0.0);
            let v2_2: f32 = format_str[4].parse().unwrap_or(0.0);
            let v3_1: f32 = format_str[5].parse().unwrap_or(0.0);
            let v3_2: f32 = format_str[6].parse().unwrap_or(0.0);

            let mut confirm = false;
            for i in &mut self.cl {
                if i.name == name {
                    confirm = true;
                    i.pos = vec![
                        Vec2 {
                            x: v1_1 as f32,
                            y: v1_2,
                        },
                        Vec2 { x: v2_1, y: v2_2 },
                        Vec2 { x: v3_1, y: v3_2 },
                    ];
                    break;
                }
            }

            if confirm == false {
                {
                    let temp = vec![
                        Vec2 {
                            x: v1_1 as f32,
                            y: v1_2,
                        },
                        Vec2 { x: v2_1, y: v2_2 },
                        Vec2 { x: v3_1, y: v3_2 },
                    ];
                    let result = Player {
                        name: name.to_string(),
                        pos: temp,
                    };
                    self.cl.push(result);
                }
            }

            for i in &mut self.cl {
                macroquad::shapes::draw_triangle_lines(
                    Vec2 {
                        x: i.pos[0].x,
                        y: i.pos[0].y,
                    },
                    Vec2 {
                        x: i.pos[1].x,
                        y: i.pos[1].y,
                    },
                    Vec2 {
                        x: i.pos[2].x,
                        y: i.pos[2].y,
                    },
                    2.,
                    macroquad::color::BLACK,
                );
            }
        }

        return true;

        //draw_triangle_lines(v1, v2, v3, thickness, color);
    }

    pub fn recv(&mut self) -> RetErr {
        // 1. Берем ссылку на Option, превращаем в Option<&TcpStream> через .as_ref()
        // 2. Если сокет есть, вызываем try_clone() уже у самого TcpStream
        let cloned_stream = match self.sock.as_ref() {
            Some(stream) => match stream.try_clone() {
                Ok(s) => s,
                Err(_) => {
                    return RetErr {
                        soul: "Ошибка клонирования сокета".to_string(),
                        status: false,
                    };
                }
            },
            None => {
                self.status = false;
                return RetErr {
                    soul: "Соединение не установлено".to_string(),
                    status: false,
                };
            }
        };

        let mut reader = BufReader::new(cloned_stream);
        let mut line = String::new();

        match reader.read_line(&mut line) {
            Ok(0) => {
                self.status = false;
                return RetErr {
                    soul: "Сервер закрыл соединение".to_string(),
                    status: false,
                };
            }
            Ok(_) => RetErr {
                soul: line.trim().to_string(),
                status: true,
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => RetErr {
                soul: "".to_string(),
                status: true,
            },
            Err(_) => {
                self.status = false;
                return RetErr {
                    soul: "Ошибка сети".to_string(),
                    status: false,
                };
            }
        }
    }
}
