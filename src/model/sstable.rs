use crate::model::key::Key;
use crate::model::value::ValueType;
use crate::model::segment::Segment;

pub struct SSTable {
    segments: Vec<Segment>,
}

impl SSTable {
    pub fn new() -> Self {
        SSTable {
            segments: Vec::new(),
        }
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    pub fn insert(&mut self, key: Key, value: ValueType) {
        panic!("Not implemented");
    }
}
