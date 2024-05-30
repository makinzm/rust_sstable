use crate::model::key::Key;
use crate::model::value::ValueType;

pub struct Segment {
    data: Vec<(Key, ValueType)>,
}

impl Segment {
    pub fn new() -> Self {
        Segment {
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: Key, value: ValueType) {
        self.data.push((key, value));
    }
}

