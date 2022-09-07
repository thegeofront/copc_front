use std::{io::{BufReader, Read, Seek, Cursor}};

use copc_rs::{reader::{CopcReader, LodSelection}, header::Header};
use wasm_bindgen::prelude::*;

type R = BufReader<Cursor<Vec<u8>>>;

#[wasm_bindgen]
pub struct Copc {
    reader: CopcReader<R>,
}

#[wasm_bindgen]
impl Copc {
    pub fn new_from_buffer(buffer: Vec<u8>) -> Self {
        let cursor = Cursor::new(buffer);
        let buffer = BufReader::new(cursor);
        let copc = CopcReader::open(buffer).expect("Could not read file");  
        
        Self { reader: copc }
    }

    pub fn get_number_of_points(&self) -> u64 {
        self.reader.get_header().number_of_points()
    }

    pub fn get_points(&mut self) {
        for point in self.reader.points(LodSelection::Level(0), None).unwrap().take(5) {
            println!("Point coordinates: ({}, {}, {})", point.x, point.y, point.z);
        }
    }
}

impl Copc {
    pub fn get_header(&self) -> &Header {
        self.reader.get_header()
    }
}

pub fn read_copc<T: Read + Seek + Send>(buffer: T) -> CopcReader<T> {
    let copc = CopcReader::open(buffer).unwrap();
    copc
}