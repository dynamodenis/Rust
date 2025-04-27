
use super::media::Media;
#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self{
        Catalog { items: vec![] }
    }
    pub fn add_item(&mut self, item: Media){
        self.items.push(item);
    }
    pub fn get_by_index(&self, index: usize) -> MighHaveAValue{
        if self.items.len() > index {
            MighHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            MighHaveAValue::ThereIsNoValue
        }
        
    }
}

pub enum MighHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    ThereIsNoValue,
}