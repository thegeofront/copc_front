mod copc;
mod utils;

pub use copc::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}

#[cfg(test)]
mod tests {
    use std::{io::BufReader, fs::{File, self}};
    use std::io::{self, Cursor};
    use crate::{read_copc, Copc};

    #[test]
    fn it_works() {

        // a normal, native way of lloading a copc
        let laz_file = BufReader::new(File::open("samples/autzen-classified.copc.laz").unwrap());
        let copc = read_copc(laz_file);

        // a 'non-native way' of deserializing 
        let vec = fs::read("samples/autzen-classified.copc.laz").unwrap();
        let copc2 = Copc::new_from_buffer(vec);
        
        assert_eq!(copc.get_header().number_of_points(), copc2.get_number_of_points());
        assert_eq!(format!("{:?}", copc.get_header()), format!("{:?}", copc2.get_header()));

        
    }
}
