use crate::Bleachers::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

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
    let mut seats_lock = seats.lock().unwrap();

    // Llamar a la función search_sites con la solicitud recibida
    let response = search_sites(request, &mut seats_lock);
    let mut resp = String::new();
    if response.is_empty() {
        resp = "Not found seats".to_string();
    } else {
        for seat in &response {
            let block = match seats_lock[seat.row_index][seat.site_index - 1].block {
                Block::VIP => "VIP",
                Block::A1 => "A1",
                Block::A2 => "A2",
                Block::B => "B",
                Block::C => "C",
            };
            resp += &format!(
                "Row number: {} Seat number {}, in block {}\n",
                seats_lock[seat.row_index][seat.site_index - 1].row,
                seat.site_index,
                block
            );
        }
    }
    // Escribir la respuesta de vuelta al cliente
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();

    // Liberar el bloqueo antes de esperar la confirmación
    drop(seats_lock);

    let mut rebase_time = false;
    // Inicializar el temporizador para la espera
    stream.set_read_timeout(Some(Duration::new(10, 0))).unwrap();
    let mut confirm_buffer = [0; 512];
    let confirmation = match stream.read(&mut confirm_buffer) {
        Ok(confirm_bytes_read) => {
            if confirm_bytes_read == 0 {
                // Si no se lee nada, asumir "no"
                rebase_time = true;
                "no".to_string()
            } else {
                // Si se recibe una respuesta dentro del tiempo, se procesa
                String::from_utf8_lossy(&confirm_buffer[..confirm_bytes_read]).trim().to_string()
            }
        }
        Err(_) => {
            // Si ocurre un timeout (o cualquier otro error), se asume "no"
            rebase_time = true;
            "no".to_string()

        }
    };

    // Re-bloquear el mutex de los asientos antes de realizar cambios
    let mut seats_lock = seats.lock().unwrap();

    // Lógica basada en la confirmación recibida
    if confirmation == "yes" && resp != "Not found seats" {
        stream.write("Successful seat purchase".to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        for seat in response.clone().clone() {
            seats_lock[seat.row_index][seat.site_index - 1].status = Status::Sold;
        }
        println!("Request finished: client selected yes.");
    } else if rebase_time {
        stream.write("Failed seat purchase: time expired .".to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        for seat in response.clone() {
            seats_lock[seat.row_index][seat.site_index - 1].status = Status::Available;
        }
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let request = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
        println!("Request finished: time expired. Cancel confirm: {}", request);
    }
    if confirmation == "no"{
        stream.write("Cancel seat purchase".to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
        for seat in response {
            seats_lock[seat.row_index][seat.site_index - 1].status = Status::Available;
        }
        println!("Request finished: client selected no.");
    }
}

