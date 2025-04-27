use std::{fs, vec};

fn extract_errors(text: &str) -> Vec<String> {
    let split_test = text.split("\n");

    let mut results = vec![];

    for line in split_test {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    let text = fs::read_to_string("logs.txt").expect("Unable to read file");
    let extracted_errors = extract_errors(&text);
    fs::write("errors.txt", extracted_errors.join("\n")).expect("Unable to write file");

    // println!("File content: {}", text);

    // let mut errors = vec![];

    // match fs::read_to_string("logs.txt"){

    //     Ok(text_that_was_read) => {
    //         let errors = extract_errors(&text_that_was_read);
    //         match fs::write("errors.text", errors.join("\n")){
    //             Ok(..) => println!("Errors written to file"),
    //             Err(error_caused) => {
    //                 println!("Error writing to file: {}", error_caused);
    //             }
                
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error reading file: {}", e);
    //     }
    // }

}
