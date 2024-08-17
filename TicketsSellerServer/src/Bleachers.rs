use std::cmp::PartialEq;

#[derive(Debug)]
enum Visibility {
    //Visibility categorie
    Excellent,
    Good,
    Regular
}

#[derive(Debug)]
enum Block {
    VIP,
    A1,
    A2,
    B,
    C
}

#[derive(Debug)]
enum Status {
    Available,
    Reserved,
    Sold
}

#[derive(Debug)]
struct Site {
    block: Block,
    visibility: Visibility,
    row: i16,
    seat: i32,
    status: Status
}


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
}


//Implementing PartialEq for the enums
impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

//Search n many seats together
pub fn searchSeats(n: i32, block: Block) -> Vec<Site>{
    let seats = matrixSeats();
    let mut result: Vec<Site> = Vec::new();
    let mut count = 0;
    for row in seats.iter(){
        for site in row.iter(){
            if site.block == block && site.status == Status::Available {
                result.push(site.clone()); //Revisar
                count += 1;
            }
            if count == n {
                return result;
            }
        }
    }
    result
}


pub fn generateAndShow(){
    let seats = matrixSeats();
    for row in seats.iter(){
        for site in row.iter(){
            println!("Block: {:?}, Visibility: {:?}, Row: {}, Seat: {} - Status: {:?}", site.block, site.visibility, site.row, site.seat, site.status);
        }
    }
}