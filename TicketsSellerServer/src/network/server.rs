use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use rand::Rng;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {
                // Convertir el buffer a un string y obtener el rango
                let received = String::from_utf8_lossy(&buffer[..size]);
                if let Ok(range) = received.trim().parse::<u32>() {
                    // Generar un número aleatorio en el rango proporcionado
                    let mut rng = rand::thread_rng();
                    let random_number = rng.gen_range(1..=range);

                    // Enviar el número al cliente
                    if let Err(e) = stream.write(format!("{}\n", random_number).as_bytes()) {
                        eprintln!("Error al escribir en el stream: {}", e);
                        return;
                    }

                    // Leer la decisión del cliente
                    let mut buffer = [0; 512];
                    if let Ok(size) = stream.read(&mut buffer) {
                        if size > 0 {
                            let decision = String::from_utf8_lossy(&buffer[..size]);
                            if decision.trim() == "si" {
                                println!("Cliente aceptó el número: {}", random_number);
                            } else {
                                println!("Cliente rechazó el número: {}", random_number);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error al leer del stream: {}", e),
    }
}

pub fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Servidor escuchando en el puerto 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error al aceptar conexión: {}", e);
            }
        }
    }
    Ok(())
}

