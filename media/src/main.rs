mod content;
use content::catalog::{Catalog, MighHaveAValue};
use content::media::Media;


fn main() {
    let good_book = Media::Book {title: String::from("The Rust Programming Language"), author: String::from("Steve Klabnik")};
    let good_movie = Media::Movie {title: String::from("Inception"), director: String::from("Christopher Nolan")};
    let good_audio_book = Media::AudioBook {title: String::from("The Hitchhiker's Guide to the Galaxy")};
    let good_podcast = Media::Podcast (1);
    let good_placeholder = Media::Placeholder;

    // println!("This is a {}", good_book.decription());
    // println!("This is a {}", good_movie.decription());
    // println!("This is a {}", good_audio_book.decription());

    let mut catalog = Catalog::new();
    catalog.add_item(good_book);
    catalog.add_item(good_movie);
    catalog.add_item(good_audio_book);
    catalog.add_item(good_podcast);
    catalog.add_item(good_placeholder);

    // match catalog.items.get(10) {
    //     Some(value) => {
    //         println!("Itam 0 is a {}", value.decription());
    //     }
    //     None => {
    //         println!("No item found");
    //     }
    // }

    // Custom OPTION
    match catalog.get_by_index(10) {
        MighHaveAValue::ThereIsAValue(value) => {
            println!("Item 0 is a {}", value.decription());
        }
        MighHaveAValue::ThereIsNoValue => {
            println!("No item found");
        }
    }
    
    println!("{:?}", catalog);
}
