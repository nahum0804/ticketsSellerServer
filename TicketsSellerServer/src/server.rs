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
    let mut resp= String::new();
    print!("\n\n");
    if response.is_empty(){
        resp = "Not foud seats".to_string();
    }else {
        for seat in &response {
            let mut block = "";
            match seats[seat.row_index][seat.site_index - 1].clone().block{
                Block::VIP=>{
                    block = "VIP";
                }
                Block::A1 =>{
                    block = "A1";
                }
                Block::A2 =>{
                    block = "A2";
                }
                Block::B =>{
                    block = "B";
                }Block::C =>{
                    block = "C";
                }
            }
            resp += &format!("Row number: {} Seat number {}, in block {}\n", seats[seat.row_index][seat.site_index - 1].row,seat.site_index,block);
        }
    }
    // Escribir la respuesta de vuelta al cliente
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();

    // Esperar la respuesta de confirmación del cliente ("yes" o "no")
    let mut confirm_buffer = [0; 512];
    let confirm_bytes_read = stream.read(&mut confirm_buffer).unwrap();
    let confirmation = String::from_utf8_lossy(&confirm_buffer[..confirm_bytes_read]).trim().to_string();

    // Imprimir la confirmación en el servidor
    println!("Confirmación recibida del cliente: {}", confirmation);

    // Opcionalmente, puedes agregar lógica adicional basada en la confirmación aquí
    if confirmation == "yes" && resp != "Not foud seats"{
        for seat in response.clone() {
             seats[seat.row_index][seat.site_index - 1].status = Status::Sold;
        }
        println!("El cliente confirmó la reserva los asientos se compraron correctamente");
    } else if confirmation == "no" || resp == "Not foud seats"{
        for seat in response {
            seats[seat.row_index][seat.site_index - 1].status = Status::Available;
        }
        println!("El cliente rechazó la reserva los asientos quedaron disponibles.");
        // Aquí podrías revertir la reserva en bleachers si es necesario
    }
}