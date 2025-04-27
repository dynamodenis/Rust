#[derive(Debug)]
pub enum Media{
    Book {title: String, author: String },
    Movie {title: String, director: String },
    AudioBook {title: String},
    Podcast (u32),
    Placeholder
}

impl Media {
    pub fn decription(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} by {}", title, author)
            }
            Media::Movie {title, director } => {
                format!("Movie {} by {}", title, director)
            }
            Media::AudioBook {title} => {
                format!("AudioBook {}", title)
            }
            Media::Podcast (id) => {
                format!("Podcast {}", id)
            }
            Media::Placeholder => {
                String::from("Placeholder")
            }
        }
    }
}