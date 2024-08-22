mod Bleachers;
mod network;
use crate::Bleachers::generateAndShow;
use crate::network::server::run_server;

fn main() {
    if let Err(e) = run_server() {
        eprintln!("Error al ejecutar el servidor: {}", e);
    }
}
