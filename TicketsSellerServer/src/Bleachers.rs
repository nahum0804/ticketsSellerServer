use std::cmp::PartialEq;
use std::{fmt, thread};
use crate::Bleachers::Status::Reserved;
use std::sync::{Arc, Mutex};


#[derive(Debug,Clone)]
pub enum Visibility {
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
pub enum Block {
    VIP,
    A1,
    A2,
    B,
    C
}

#[derive(Debug,Clone)]
pub enum Status {
    Available,
    Reserved,
    Sold
}

#[derive(Debug,Clone)]
pub struct Site {
    block: Block,
    visibility: Visibility,
    row: i16,
    seat: i32,
    status: Status
}


pub fn matrixSeats() -> Vec<Vec<Site>> {
    let seats = vec![
        vec![Site { block: Block::VIP, visibility: Visibility::Good, row: 1, seat: 1, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 2, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 1, seat: 9, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Good, row: 1, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Good, row: 2, seat: 1, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 2, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Excellent, row: 2, seat: 9, status: Status::Available },
             Site { block: Block::VIP, visibility: Visibility::Good, row: 2, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Good, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Good, row: 3, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Good, row: 4, seat: 1, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 2, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 3, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 4, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 5, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 6, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 7, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 8, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 4, seat: 9, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Good, row: 4, seat: 10, status: Status::Available }],
        vec![Site { block: Block::VIP, visibility: Visibility::Good, row: 5, seat: 1, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 2, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 3, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 4, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 5, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 6, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 7, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 8, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Excellent, row: 5, seat: 9, status: Status::Available },
                Site { block: Block::VIP, visibility: Visibility::Good, row: 5, seat: 10, status: Status::Available }],
     vec![Site { block: Block::A1, visibility: Visibility::Good, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 1, seat: 10, status: Status::Available }],
        vec![Site { block: Block::A1, visibility: Visibility::Good, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 2 , seat: 10, status: Status::Available }],
        vec![Site { block: Block::A1, visibility: Visibility::Good, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Excellent, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::A1, visibility: Visibility::Good, row: 3, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::A2, visibility: Visibility::Good, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 1, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::A2, visibility: Visibility::Good, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 2, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::A2, visibility: Visibility::Good, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Excellent, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::A2, visibility: Visibility::Good, row: 3, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::B, visibility: Visibility::Regular, row: 1, seat: 1, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 2, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 3, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 4, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 5, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 6, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 7, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 8, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 1, seat: 9, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Regular, row: 1, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::B, visibility: Visibility::Regular, row: 2, seat: 1, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 2, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 3, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 4, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 5, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 6, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 7, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 8, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 2, seat: 9, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Regular, row: 2, seat: 10, status: Status::Available }],
        vec![ Site { block: Block::B, visibility: Visibility::Regular, row: 3, seat: 1, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 2, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 3, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 4, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 5, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 6, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 7, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 8, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Good, row: 3, seat: 9, status: Status::Available },
                Site { block: Block::B, visibility: Visibility::Regular, row: 3, seat: 10, status: Status::Available }],
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
}

#[derive(Debug,Clone)]
pub struct Sel_site {
    row_index: usize,
    site_index: usize
}
impl fmt::Display for Sel_site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Row: {}, Seat: {}", self.row_index, self.site_index)
    }
}

fn map2<T, U, F>(lista:&Vec<T>,bleachers:&mut Vec<Vec<Site>>, funcion: F) -> Vec<U>
where
    F: Fn(&T,&mut Vec<Vec<Site>>) -> U,
{
    let mut resultado: Vec<U> = Vec::new();
    for item in lista {
        resultado.push(funcion(item,bleachers));
    }
    resultado
}

pub fn points(blok:&Vec<Sel_site>, bleachers:&mut Vec<Vec<Site>>) ->i32{
    let mut points = 0;
    for site in blok{
        match bleachers[site.row_index][site.site_index-1].visibility{
            Visibility::Excellent =>{
                points+=3;
            }
            Visibility::Good =>{
                points+=2;
            }
            Visibility::Regular => {
                points += 1;
            },
            _ => {points+=0;}
        }
    }
    return points;
}



fn block_congruents (block2:& Vec<Sel_site>, block1:&Vec<Sel_site>) ->bool{
    if block2[0].site_index<=block1[0].site_index && block1.last().unwrap().site_index<=block2.last().unwrap().site_index{
        return true;
    }
    return false;
}

// Trabajar aquí - Mejorar la busqueda en otras filas (Buscar posibles campos)
pub fn search_sites(request:String, bleachers:&mut Vec<Vec<Site>>)->String {

    let mut collection_request: Vec<&str> = request.split('/').collect();
    let seats: u32 = collection_request[0].parse().expect("Number Invalid");
    let limit_seats = 15;
    let mut block = collection_request[1];
    let mut selected_seats: Vec<Sel_site> = Vec::new();
    let mut avalible_seats = 0;
    let mut possible_blocks: Vec<Vec<Sel_site>> = Vec::new();

    if seats > limit_seats {
        return "The number of seats exceeds the limit".to_string();
    }
    let options = vec!["VIP","A1","A2","B","C"];
    let mut found = 0;
    while found<5 {
        match block {
            "VIP" => {
                for index_row in 0..5 {
                    avalible_seats = 0;
                    for seat in &mut bleachers[index_row] {
                        match seat.status {
                            Status::Available => {
                                avalible_seats += 1;
                                selected_seats.push(Sel_site { row_index: index_row, site_index: seat.seat as usize });
                            }
                            Status::Sold => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                            Status::Reserved => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                        }
                        if avalible_seats == seats as usize {
                            possible_blocks.push(selected_seats.clone());
                            avalible_seats = 0;
                            selected_seats.clear();
                        }
                    }
                    if avalible_seats >= (seats as usize / 2) {
                        possible_blocks.push(selected_seats.clone());
                    }
                    selected_seats.clear();
                }
            }
            "A1" => {
                for index_row in 5..8 {
                    avalible_seats = 0;
                    for seat in &mut bleachers[index_row] {
                        match seat.status {
                            Status::Available => {
                                avalible_seats += 1;
                                selected_seats.push(Sel_site { row_index: index_row, site_index: seat.seat as usize });
                            }
                            Status::Sold => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                            Status::Reserved => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                        }
                        if avalible_seats == seats as usize {
                            possible_blocks.push(selected_seats.clone());
                            avalible_seats = 0;
                            selected_seats.clear();
                        }
                    }
                    if avalible_seats >= (seats as usize / 2) {
                        possible_blocks.push(selected_seats.clone());
                    }
                    selected_seats.clear();
                }
            }
            "A2" => {
                for index_row in 8..11 {
                    avalible_seats = 0;
                    for seat in &mut bleachers[index_row] {
                        match seat.status {
                            Status::Available => {
                                avalible_seats += 1;
                                selected_seats.push(Sel_site { row_index: index_row, site_index: seat.seat as usize });
                            }
                            Status::Sold => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                            Status::Reserved => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                        }
                        if avalible_seats == seats as usize {
                            possible_blocks.push(selected_seats.clone());
                            avalible_seats = 0;
                            selected_seats.clear();
                        }
                    }
                    if avalible_seats >= (seats as usize / 2) {
                        possible_blocks.push(selected_seats.clone());
                    }
                    selected_seats.clear();
                }
            }
            "B" => {
                for index_row in 11..14 {
                    avalible_seats = 0;
                    for seat in &mut bleachers[index_row] {
                        match seat.status {
                            Status::Available => {
                                avalible_seats += 1;
                                selected_seats.push(Sel_site { row_index: index_row, site_index: seat.seat as usize });
                            }
                            Status::Sold => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                            Status::Reserved => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                        }
                        if avalible_seats == seats as usize {
                            possible_blocks.push(selected_seats.clone());
                            avalible_seats = 0;
                            selected_seats.clear();
                        }
                    }
                    if avalible_seats >= (seats as usize / 2) {
                        possible_blocks.push(selected_seats.clone());
                    }
                    selected_seats.clear();
                }
            }
            "C" => {
                for index_row in 14..17 {
                    avalible_seats = 0;
                    for seat in &mut bleachers[index_row] {
                        match seat.status {
                            Status::Available => {
                                avalible_seats += 1;
                                selected_seats.push(Sel_site { row_index: index_row, site_index: seat.seat as usize });
                            }
                            Status::Sold => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                            Status::Reserved => {
                                if avalible_seats >= (seats as usize / 2) {
                                    possible_blocks.push(selected_seats.clone());
                                }
                                avalible_seats = 0;
                                selected_seats.clear();
                            }
                        }
                        if avalible_seats == seats as usize {
                            possible_blocks.push(selected_seats.clone());
                            avalible_seats = 0;
                            selected_seats.clear();
                        }
                    }
                    if avalible_seats >= (seats as usize / 2) {
                        possible_blocks.push(selected_seats.clone());
                    }
                    selected_seats.clear();
                }
            }
            _ => {
                println!("Block not found");
                return "none".to_string();
            }
        }

        let mut possible_resp:Vec<Vec<Sel_site>> = Vec::new();
        let mut ret = String::new();

        for option in possible_blocks.iter() {
            if option.len() == seats as usize {
                possible_resp.push(option.clone());
            }
        }
        if !possible_resp.is_empty(){
            let listPoints = map2(&possible_resp.clone(),bleachers,points);
            let mut mayor_points=0;
            let mut index=0;
            for index_points in 0..listPoints.iter().len() {
                if listPoints[index_points]>mayor_points { mayor_points=listPoints[index_points]; index=index_points}
            }
            for x in possible_resp[index].clone() {
                bleachers[x.row_index][x.site_index - 1].status = Reserved;
                ret += &format!("Row number: {} Seat number {}, in block {}\n", bleachers[x.row_index][x.site_index - 1].row, x.site_index, block);
            }
            return ret;
        }

        //case 2 (2 bloques seguidos)
        let cuant_block1 = seats/2;
        let mut cuant_block2 =0;
        if seats % 2 !=0{
            cuant_block2 = (seats/2)+1;
        }else {
            cuant_block2 = cuant_block1;
        }

        for option_index in 0..possible_blocks.len() {
            if block == "VIP"{
                if possible_blocks[option_index].len() >= cuant_block2 as usize {
                    if possible_blocks[option_index][0].row_index == 0{

                        if possible_blocks[option_index+1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index+1]){

                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index+1].clone());
                        }
                    }
                    else if possible_blocks[option_index][0].row_index == 4 {
                        println!("Entro en la ultima fila de VIP");
                        if possible_blocks.len() != 1{
                            if possible_blocks[option_index-1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index-1]){
                                possible_resp.push(possible_blocks[option_index].clone());
                                possible_resp.push(possible_blocks[option_index-1].clone());
                            }
                        }

                    }
                    else {
                        if possible_blocks[option_index+1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index+1]){
                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index+1].clone());
                        }
                        else if possible_blocks[option_index-1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index-1]){
                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index-1].clone());
                        }
                    }
                }
            }else {
                if possible_blocks[option_index].len() >= cuant_block2 as usize {
                    if possible_blocks[option_index][0].row_index == 0 || option_index == 0{

                        if possible_blocks[option_index+1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index+1]){
                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index+1].clone());
                        }
                    }
                    else if possible_blocks[option_index][0].row_index == 2 || option_index == possible_blocks.len()-1{
                        if possible_blocks[option_index-1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index-1]){
                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index-1].clone());
                        }
                    }
                    else {
                        if possible_blocks[option_index+1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index+1]){
                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index+1].clone());
                        }
                        else if possible_blocks[option_index-1].len() >= cuant_block1 as usize && block_congruents(&possible_blocks[option_index],&possible_blocks[option_index-1]){
                            possible_resp.push(possible_blocks[option_index].clone());
                            possible_resp.push(possible_blocks[option_index-1].clone());
                        }
                    }
                }
            }

        }
        if !possible_resp.is_empty(){
            for x in (0..possible_resp.len()).step_by(2){
                if possible_resp[x].len()>cuant_block2 as usize{
                    let temp =possible_resp[x][0..(cuant_block2 as usize)].to_vec();
                    possible_resp[x] = temp.clone();
                }
                if possible_resp[x+1].len()>cuant_block1 as usize{
                    let temp =possible_resp[x+1][0..(cuant_block1 as usize)].to_vec();
                    possible_resp[x+1] = temp.clone();
                }
            }
            let listPoints = map2(&possible_resp.clone(),bleachers,points);
            let mut mayor_points=0;
            let mut index=0;
            for index_points in (0..listPoints.iter().len()).step_by(2) {
                let plus_points = listPoints[index_points]+listPoints[index_points+1];
                if plus_points>mayor_points { mayor_points=plus_points; index=index_points}
            }
            for x in possible_resp[index].clone(){
                bleachers[x.row_index][x.site_index - 1].status = Reserved;
                ret += &format!("Row number: {} Seat number {}, in block {}\n", bleachers[x.row_index][x.site_index - 1].row, x.site_index, block);
            }
            for x in possible_resp[index+1].clone(){
                bleachers[x.row_index][x.site_index - 1].status = Reserved;
                ret += &format!("Row number: {} Seat number {}, in block {}\n", bleachers[x.row_index][x.site_index - 1].row, x.site_index, block);
            }
            return ret;
        }
        for blockTemp in 0..5{

            if block == options[blockTemp]{
                if block == "C"{
                    block = "VIP";
                }else{
                    block = options[blockTemp+1];

                }
                println!("\nNo se encontro boloque se cambio a {}\n",block);
                found+=1;
                break;
            }
        }
    }
    return "none".to_string();

}