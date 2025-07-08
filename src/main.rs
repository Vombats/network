use std::io;
use std::process::Command;
fn main() {
    println!("\nПрограмма для получения IP-адреса и скорости подключения");

    // Получаем IP-адрес с помощью ident.me
    match get_ip_address() {
        Ok(ip_address) => println!("\nВаш текущий IP-адрес: {}", ip_address),
        Err(e) => println!("Ошибка при получении IP-адреса: {}", e),
    }

    // Получаем информацию о скорости подключения
    match get_network_speed() {
        Ok(speed) => println!("\nСкорость подключения: {} Мбит/с", speed),
        Err(e) => println!("Ошибка при получении скорости: {}", e),
    }
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
