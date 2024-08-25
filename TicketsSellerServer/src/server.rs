use crate::Bleachers::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};

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
    // Buffer para almacenar la solicitud del cliente
    let mut buffer = [0; 512];

    // Leer la solicitud del cliente
    let bytes_read = stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

    // Obtener un bloqueo mutable para modificar los asientos
    let mut seats = seats.lock().unwrap();

    // Llamar a la función search_sites con la solicitud recibida
    let response = search_sites(request, &mut seats);

    // Escribir la respuesta de vuelta al cliente
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // Esperar la respuesta de confirmación del cliente ("yes" o "no")
    let mut confirm_buffer = [0; 512];
    let confirm_bytes_read = stream.read(&mut confirm_buffer).unwrap();
    let confirmation = String::from_utf8_lossy(&confirm_buffer[..confirm_bytes_read]).trim().to_string();

    // Imprimir la confirmación en el servidor
    println!("Confirmación recibida del cliente: {}", confirmation);

    // Opcionalmente, puedes agregar lógica adicional basada en la confirmación aquí
    if confirmation == "yes" {
        println!("El cliente confirmó la reserva.");
    } else if confirmation == "no" {
        println!("El cliente rechazó la reserva.");
        // Aquí podrías revertir la reserva en bleachers si es necesario
    }
}