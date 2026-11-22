use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
mod logger;

#[derive(Clone, Debug)]
struct Message {
    addr: SocketAddr,
    text: String,
}

#[tokio::main]
async fn main() {
    let logger = logger::Logger::new("log.txt").unwrap();
    // 1. Запускаем TCP-сервер на локальном порту 8080
    let listener = match TcpListener::bind("0.0.0.0:8080").await {
        Ok(ok) => ok,
        Err(err) => {
            logger.error(format!("Сервер не запущен: {}", err.to_string()));
            panic!("{err}");
        }
    };
    logger.info("Сервер запущен на 127.0.0.1:8080");

    // 2. Создаем broadcast-канал для ретрансляции сообщений всем клиентам
    let (tx, _) = broadcast::channel::<Message>(100);

    // 3. Главный цикл ACCEPT — здесь мы ждем и принимаем новые сетевые подключения
    loop {
        let logger_cl = logger.clone();
        let (socket, addr) = match listener.accept().await {
            Ok(ok) => ok,
            Err(err) => {
                logger.error(format!("Клиент не принят: {}", &err.to_string()));
                continue;
            }
        };
        logger_cl.info(format!("Клиент подключен: {}", socket.peer_addr().unwrap()));

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // 4. Спавним отдельную асинхронную задачу для каждого подключившегося клиента
        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            // Задача А: Отправка сообщений из broadcast-канала в TCP-сокет клиента
            tokio::spawn(async move {
                while let Ok(msg) = rx.recv().await {
                    // Игнорируем сообщения, которые этот же клиент сам и отправил
                    if msg.addr != addr {
                        if writer.write_all(msg.text.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                }
            });

            // Задача Б: Чтение входящих строк из TCP-сокета и отправка их в broadcast
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break, // Клиент закрыл соединение
                    Ok(_) => {
                        let msg = Message {
                            addr,
                            text: line.clone(),
                        };
                        //logger_cl.info(format!("{} : {}", msg.text.trim(), addr.to_string()));
                        // Рассылаем сообщение абсолютно всем подписчикам канала
                        let _ = tx.send(msg);
                    }
                    Err(_) => break, // Ошибка сети
                }
            }

            logger_cl.info(format!("Клиент {} отключился", addr));
        });
    }
}
