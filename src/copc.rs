use std::{io::{BufReader, Read, Seek, Cursor}};

use copc_rs::{reader::{CopcReader, LodSelection}, header::Header};
use las::point::Classification;
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

    pub fn get_lod_zero(&mut self, classification: u8) -> Vec<f64> {
        let mut points = Vec::new();
        let class = Classification::new(classification).expect("could not parse classification"); 
        for point in self.reader.points(LodSelection::Level(0), None).unwrap() {
            if point.classification == class {
                points.push(point.x);
                points.push(point.y);
                points.push(point.z);
            }
        }
        points
    }

    pub fn header(&self) {
        let header = self.reader.get_header();
    }

    pub fn get_point_count(&self) -> u64 {
        self.get_header().number_of_points()
    }
}

#[wasm_bindgen]
pub struct Class(Classification);

#[wasm_bindgen]
pub struct DumbHelpers {}

#[wasm_bindgen]
impl DumbHelpers {

    pub fn scale_array(arr: Vec<f64>, scalar: f64) -> Vec<f64> {
        let mut points = Vec::new();
        for item in arr {
            points.push(item * scalar);
        }
        points
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

