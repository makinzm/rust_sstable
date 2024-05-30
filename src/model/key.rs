pub struct Key {
    size: usize,
    names: Vec<String>,
    types: Vec<KeyType>,
    priorities: Vec<u8>,
}

pub enum KeyType {
    String,
    Number,
}

