use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

/// Уровень сообщения.
///
/// Чем выше уровень — тем более серьёзная проблема.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    /// Превращаем уровень в обычный текст.
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Одно сообщение логгера.
///
/// Именно такие структуры будут передаваться через mpsc channel.
struct LogMessage {
    level: LogLevel,
    message: String,
}

/// Публичный логгер.
///
/// Его можно спокойно клонировать и передавать
/// в разные потоки.
///
/// Внутри находится только Sender.
///
/// Сам файл находится ВНЕ Logger — им владеет
/// отдельный поток.
#[derive(Clone)]
pub struct Logger {
    sender: mpsc::Sender<LogMessage>,
}

#[allow(dead_code)]
impl Logger {
    /// Создаём новый Logger.
    ///
    /// `path` — путь к файлу логов.
    ///
    /// Например:
    ///
    /// Logger::new("logs/server.log")
    pub fn new<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        // Создаём канал.
        //
        // 1024 — максимальное количество сообщений,
        // которое может находиться в очереди.
        //
        // ВАЖНО:
        // Sender можно клонировать.
        // Receiver будет только один.
        let (sender, receiver) = mpsc::channel::<LogMessage>();

        // Открываем файл ДО запуска logger thread.
        //
        // create(true)  -> создать файл, если его нет.
        // append(true)  -> новые сообщения добавляются
        //                  в конец файла.
        // write(true)   -> разрешаем запись.
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(path)?;

        // Запускаем отдельный поток.
        //
        // Это ЕДИНСТВЕННЫЙ поток, который будет:
        //
        // 1. читать сообщения;
        // 2. писать их в файл;
        // 3. выводить их в консоль.
        //
        // Поэтому никакой гонки при записи не возникает.
        thread::spawn(move || {
            // recv() блокирует поток, пока есть отправители.
            // Когда ВСЕ Sender уничтожены, recv() вернёт Err,
            // и цикл завершится.
            while let Ok(log) = receiver.recv() {
                // Получаем текущее локальное время.
                let time = Local::now().format("%Y-%m-%d %H:%M:%S");

                // Формируем одну строку лога.
                //
                // Например:
                //
                // [2026-09-22 01:20:15] [INFO] Server started
                let line = format!("[{}] [{}] {}\n", time, log.level.as_str(), log.message);

                // ====================================================
                // ЗАПИСЬ В ФАЙЛ
                // ====================================================
                //
                // write_all() — обычная синхронная запись.
                // Мы уже в отдельном потоке, UI/основной поток
                // это не блокирует.
                if let Err(error) = file.write_all(line.as_bytes()) {
                    eprintln!("Logger file error: {}", error);
                }

                // ====================================================
                // ВЫВОД В КОНСОЛЬ
                // ====================================================
                //
                // Здесь println! вызывает только ОДИН поток,
                // поэтому строки не перемешиваются между собой.
                print!("{}", line);
            }

            // Сюда попадём, когда ВСЕ Sender уничтожены.
            //
            // Перед завершением пытаемся сбросить буфер файла.
            if let Err(error) = file.flush() {
                eprintln!("Logger flush error: {}", error);
            }
        });

        // Возвращаем пользователю только Sender.
        //
        // Сам Receiver уже принадлежит logger thread.
        Ok(Self { sender })
    }

    /// Отправить сообщение в лог.
    ///
    /// Эта функция синхронная.
    ///
    /// Если очередь заполнена, .send() подождёт,
    /// пока в очереди появится место.
    fn log(&self, level: LogLevel, message: String) {
        let log = LogMessage { level, message };

        // Отправляем сообщение в канал.
        //
        // Здесь НЕТ записи в файл.
        // Здесь НЕТ записи в консоль.
        //
        // Мы просто передаём сообщение logger thread.
        //
        // Если logger thread уже завершился, send() вернёт ошибку.
        let _ = self.sender.send(log);
    }

    /// TRACE
    pub fn trace(&self, message: impl Into<String>) {
        self.log(LogLevel::Trace, message.into());
    }

    /// DEBUG
    pub fn debug(&self, message: impl Into<String>) {
        self.log(LogLevel::Debug, message.into());
    }

    /// INFO
    pub fn info(&self, message: impl Into<String>) {
        self.log(LogLevel::Info, message.into());
    }

    /// WARNING
    pub fn warn(&self, message: impl Into<String>) {
        self.log(LogLevel::Warn, message.into());
    }

    /// ERROR
    pub fn error(&self, message: impl Into<String>) {
        self.log(LogLevel::Error, message.into());
    }
}
