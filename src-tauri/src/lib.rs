use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};

#[tauri::command]
fn connect_server() {
    let port_number = 33117;
    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 10, 58)), port_number);

    let mut stream = match TcpStream::connect(server_address) {
        Ok(stream) => stream,
        Err(_) => {
            println!("\nConnection Failed \n");
            return;
        }
    };

    let message = "Hello from client";
    match stream.write_all(message.as_bytes()) {
        Ok(_) => println!("Message sent to server"),
        Err(e) => {
            println!("Error sending message: {}", e);
            return;
        }
    }

    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("Received message is \"{}\"", received_message);
        }
        Err(e) => {
            eprintln!("Error reading from server: {}", e);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![connect_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
