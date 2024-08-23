use std::cmp::PartialEq;
use std::thread;
use crate::Bleachers::Status::Reserved;

#[derive(Debug,Clone)]
enum Visibility {
    //Visibility categorie
    Excellent,
    Good,
    Regular
}

impl PartialEq for Visibility {
    fn eq(&self, other: &Self) -> bool {
        // Compara las variantes directamente
        match (self, other) {
            (Visibility::Excellent, Visibility::Excellent) => true,
            (Visibility::Good, Visibility::Good) => true,
            (Visibility::Regular, Visibility::Regular) => true,
            _ => false,
        }
    }
}

#[derive(Debug,Clone)]
enum Block {
    VIP,
    A1,
    A2,
    B,
    C
}

#[derive(Debug,Clone)]
enum Status {
    Available,
    Reserved,
    Sold
}

#[derive(Debug,Clone)]
struct Site {
    block: Block,
    visibility: Visibility,
    row: i16,
    seat: i32,
    status: Status
}

struct Bleachers {
    seats: Mutex<Vec<Vec<Site>>>,
}

impl Bleachers {
    fn new() -> Self {
        Self {
            seats: Mutex::new(vec![
                vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 1, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 10, status: Status::Available }],
                vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 1, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 10, status: Status::Available }],
                vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 1, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 10, status: Status::Available }],
                vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 1, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 2, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 3, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 4, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 5, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 6, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 7, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 8, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 9, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 10, status: Status::Available }],
                vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 1, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 2, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 3, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 4, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 5, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 6, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 7, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 8, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 9, status: Status::Available },
                     Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 10, status: Status::Available }],
                vec![Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 1, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 10, status: Status::Available }],
                vec![Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 1, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 2 , seat: 10, status: Status::Available }],
                vec![Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 1, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                     Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 1, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 1, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 1, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                      Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 1, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 2, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 3, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 4, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 5, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 6, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 7, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 8, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 9, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 1, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 2, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 3, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 4, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 5, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 6, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 7, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 8, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 9, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 1, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 2, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 3, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 4, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 5, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 6, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 7, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 8, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 9, status: Status::Available },
                      Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 1, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 2, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 3, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 4, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 5, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 6, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 7, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 8, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 9, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 1, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 2, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 3, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 4, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 5, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 6, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 7, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 8, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 9, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 10, status: Status::Available }],
                vec![ Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 1, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 2, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 3, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 4, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 5, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 6, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 7, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 8, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 9, status: Status::Available },
                      Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 10, status: Status::Available }]
            ]), // Inicializa con algunos datos
        }
    }

    fn update_seats(&self) {
        // Bloquea el mutex para obtener un `MutexGuard`
        let mut seats_guard = self.seats.lock().unwrap();

        // Itera sobre las filas de asientos
        for row in seats_guard.iter_mut() {
            for seat in row.iter_mut() {
                // Modifica cada 'seat' según sea necesario
            }
        }
    }
}

/*
fn matrixSeats() -> Vec<Vec<Site>> {
    let seats = vec![
        vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 1, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 1, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 1, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 2, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 3, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 4, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 5, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 6, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 7, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 8, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 9, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 1, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 2, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 3, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 4, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 5, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 6, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 7, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 8, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 9, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 10, status: Status::Available }],
     vec![Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 10, status: Status::Available }],
        vec![Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2 , seat: 10, status: Status::Available }],
        vec![Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 1, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 2, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::C, visibility: Visibility::Regular, row: 3, seat: 10, status: Status::Available }]
        ];
    seats
}*/

#[derive(Debug,Clone)]
struct Sel_site {
    row_index: usize,
    site_index: usize
}

fn serch_row_block(bleachers:&mut Vec<Vec<Site>>){

}

fn majority(m_visibility:Visibility, blok:&Vec<Sel_site>, bleachers:&mut Vec<Vec<Site>>) ->bool{
    let mut count = 0;
    for x in blok{
        if(bleachers[x.row_index][x.site_index-1].visibility==m_visibility){
            count +=1;
        }
    }
    if count as usize>=(blok.len()/2){
        return true;
    }
    return false;
}

pub fn search_sites(request:String, bleachers:&mut Vec<Vec<Site>>)->String{
    let mut collection_request:Vec<&str> = request.split('/').collect();
    let seats:u32 = collection_request[0].parse().expect("Number Invalid");
    let block = collection_request[1];
    let mut selected_seats: Vec<Sel_site> = Vec::new();
    let mut avalible_seats = 0;
    let mut possible_blocks: Vec<Vec<Sel_site>> = Vec::new();
    if(block=="VIP"){
        for index_row in 0..4 {
            avalible_seats = 0;
            for seat in &mut bleachers[index_row] {
                match seat.status {
                    Status::Available => {
                        avalible_seats += 1;
                        selected_seats.push( Sel_site{site_index:seat.seat as usize, row_index:index_row});
                    }
                    Status::Sold => {
                        avalible_seats = 0;
                        if selected_seats.len()>=(seats as usize/2){
                            possible_blocks.push(selected_seats.clone());
                        }
                        selected_seats.clear();
                    }
                    Status::Reserved => {
                        avalible_seats = 0;
                        if selected_seats.len()>=(seats as usize/2){
                            possible_blocks.push(selected_seats.clone());
                        }
                        selected_seats.clear();
                    }
                }
                if avalible_seats == seats {
                    possible_blocks.push(selected_seats.clone());
                }
            }

        }
        for option in possible_blocks.iter().rev() {
            if(option.len()==seats as usize && majority(Visibility::Excellent,option,bleachers)){
                println!("{}",option.len());
                let mut ret=String::new();
                for x in option{
                    bleachers[x.row_index][x.site_index-1].status=Reserved;
                    ret += &format!("Row number: {} Seat number {}, in block VIP\n", bleachers[x.row_index][x.site_index-1].row, x.site_index);
                }
                return ret;
            }
        }
    }
    return "none".to_string();
}
use std::sync::{Arc, Mutex};
pub fn generateAndShow(){
    /*
    let seats = matrixSeats();
    let seats = Arc::new(Mutex::new(seats));

    let mut handles = vec![];

    for i in 0..10 {
        let seats = Arc::clone(&seats);
        let handle = thread::spawn(move || {
            let mut seats = seats.lock().unwrap();
            // Ejemplo de operación en el vector
            for row in &mut seats {
                for site in row {
                    if site.status == Status::Available {
                        site.status = Status::Reserved; // Marca el asiento como reservado
                    }
                }
            }
            println!("Hilo {} ha actualizado los asientos.", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Todos los hilos han terminado.");

    -------------------------------------------------------
    use std::sync::Mutex;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct Site {
    // Define los campos de tu estructura Site aquí
}

struct Bleachers {
    seats: Mutex<Vec<Vec<Site>>>,
}

impl Bleachers {
    fn new() -> Self {
        Self {
            seats: Mutex::new(vec![vec![Site {}; 10]; 10]), // Inicializa con algunos datos
        }
    }

    fn update_seats(&self) {
        // Bloquea el mutex para obtener un `MutexGuard`
        let mut seats_guard = self.seats.lock().unwrap();

        // Itera sobre las filas de asientos
        for row in seats_guard.iter_mut() {
            for seat in row.iter_mut() {
                // Modifica cada 'seat' según sea necesario
            }
        }
    }
}

fn main() {
    let bleachers = Arc::new(Bleachers::new());
    let mut handles = vec![];

    for _ in 0..5 {
        let bleachers_clone = Arc::clone(&bleachers);
        let handle = thread::spawn(move || {
            bleachers_clone.update_seats();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

     */
}