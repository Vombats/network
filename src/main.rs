use std::io;
use std::process::{Command};

fn main() {
    // Show loading popup immediately
    let loading_popup = Command::new("zenity")
        .arg("--info")
        .arg("--title=Сетевая информация")
        .arg("--text=Загрузка данных...")
        .arg("--width=300")
        .spawn();

    let mut message = String::new();

    // Get IP address
    match get_ip_address() {
        Ok(ip_address) => message.push_str(&format!("Ваш текущий IP-адрес: {}\n\n", ip_address)),
        Err(e) => message.push_str(&format!("Ошибка при получении IP-адреса: {}\n\n", e)),
    }

    // Get network speed
    match get_network_speed() {
        Ok(speed) => message.push_str(&format!("Скорость подключения: {} Мбит/с", speed)),
        Err(e) => message.push_str(&format!("Ошибка при получении скорости: {}", e)),
    }

    // Close the loading popup if it's still open
    if let Ok(mut child) = loading_popup {
        let _ = child.kill();
    }

    // Show results popup
    let _ = Command::new("zenity")
        .arg("--info")
        .arg("--title=Сетевая информация")
        .arg("--text")
        .arg(&message)
        .arg("--width=300")
        .output();
}

fn get_ip_address() -> Result<String, io::Error> {
    let output = Command::new("curl")
        .arg("ident.me")
        .output()
        .map_err(|_e: io::Error| {
            io::Error::new(io::ErrorKind::Other, "Ошибка выполнения команды curl")
        })?;

    let ip_str = String::from_utf8_lossy(&output.stdout);
    Ok(ip_str.trim().to_string())
}

fn get_network_speed() -> Result<f64, io::Error> {
    let output = Command::new("speedtest-cli")
        .arg("--simple")
        .output()
        .map_err(|_e| {
            io::Error::new(
                io::ErrorKind::Other,
                "Ошибка выполнения команды speedtest-cli",
            )
        })?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.starts_with("Download:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let replaced = parts[1].replace("Mbit/s", "");
                let speed_str = replaced.trim();
                return speed_str.parse::<f64>().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Ошибка преобразования скорости")
                });
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Скорость не найдена",
    ))
}
