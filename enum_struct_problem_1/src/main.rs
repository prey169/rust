#[derive(Debug)]
struct Item {
    id: u32,
    title: String,
    year: u32,
    type_: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

impl Item {
    fn new(id: u32, title: String, year: u32, type_: ItemType) -> Self {
        Self {
            id,
            title,
            year,
            type_,
        }
    }

    fn display_item_info(&self){
        println!("Item ID: {:?}", &self.id);
        println!("Title: {:?}", &self.title);
        println!("Publication Year: {:?}", &self.year);
        println!("Publication Type: {:?}", &self.type_);
    }
}

fn main() {
    let book = Item::new(222, "Book!".to_string(), 1992, ItemType::Book);
    book.display_item_info();
    let mag = Item::new(111, "Mags!".to_string(), 1993, ItemType::Magazine);
    mag.display_item_info();
}
