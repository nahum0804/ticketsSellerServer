use crate::Bleachers::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use std::sync::MutexGuard;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let seats = Arc::new(Mutex::new(matrixSeats()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let seats = Arc::clone(&seats);

        thread::spawn(move || {
            handle_request(stream, seats);
        });
    }
}

fn handle_request(mut stream: TcpStream, seats: Arc<Mutex<Vec<Vec<Site>>>>) {
    // Buffer de entrada y lectura de la solicitud
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

    let mut seats_lock = seats.lock().unwrap();

    if request == "mapping" {
        // Enviar respuesta con asientos no disponibles
        let non_available_sites = get_non_available_sites(&mut seats_lock);
        let resp = if non_available_sites.is_empty() {
            "No unavailable seats found".to_string()
        } else {
            non_available_sites
        };
        stream.write_all(resp.as_bytes()).unwrap();
    } else {
    let response = get_better_three(request, &mut seats_lock);

    // Convertir la respuesta a JSON
    let resp = format_seats_as_json(&response);

    // Enviar la respuesta en formato JSON al cliente
    stream.write_all(resp.as_bytes()).unwrap();

    drop(seats_lock); // liberar mutex antes de esperar confirmación
    let confirmation = await_confirmation(&mut stream);

    // Confirmar, cancelar o liberar asientos
    process_confirmation(confirmation, response, seats);
    }
}

fn format_seats_as_json(response: &Vec<Vec<Sel_site>>) -> String {
    serde_json::to_string(response).unwrap_or_else(|_| "[]".to_string())
}

fn await_confirmation(stream: &mut TcpStream) -> i8 {
    // Esperar confirmación y manejar expiración
    stream.set_read_timeout(Some(Duration::new(120, 0))).unwrap();
    let mut confirm_buffer = [0; 512];
    match stream.read(&mut confirm_buffer) {
        Ok(confirm_bytes_read) if confirm_bytes_read > 0 => {
            String::from_utf8_lossy(&confirm_buffer[..confirm_bytes_read])
                .trim()
                .parse::<i8>()
                .unwrap_or(-1)
        }
        _ => -1, // Timeout o error
    }
}

fn process_confirmation(
    confirmation: i8,
    response: Vec<Vec<Sel_site>>,
    seats: Arc<Mutex<Vec<Vec<Site>>>>
) {
    let mut seats_lock = seats.lock().unwrap();
    match confirmation {
        -1 => gestor_better_three(-1, response, &mut seats_lock),
        _ => gestor_better_three(confirmation, response, &mut seats_lock),
    };
}


