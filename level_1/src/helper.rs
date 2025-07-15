use std::time::{SystemTime, UNIX_EPOCH};

// pub struct RNG {
//     state: u8,
// }

// impl RNG {
//     // fn new()->Self{
//     // }

//     // pub fn generate_u8(&mut self) -> u8 {
//     //     self.state = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u8;
//     //     let mut x = self.state;

//     //     x ^= x << 13;
//     //     x ^= x >> 7;
//     //     x ^= x << 17;
//     //     self.state = x;
//     //     x
//     // }
// }

pub fn generate_u8() -> u8 {
    let mut x = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u8;

    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}
