
fn print_elements(elements: &[String]){
    // for element in elements{
    //     println!("{}", element);
    // }

    elements.iter().for_each(|el| println!("{}", el) );
}

fn shortened_string(elements: &mut Vec<String>){
    elements.iter_mut()
        .for_each(|el| el.truncate(1));
}

fn to_uppercase(elements:&[String]) -> Vec<String>{
    elements.iter().map(|el| el.to_uppercase()).collect()
}

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el))
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements.iter().map(|el| el.chars().map(|c| c.to_string()).collect()).collect()
}

fn find_color_or_default(elements: &[String], color: &str, fallback: &str) -> String {
    // elements.iter().find(|el| el.contains(color)).cloned().unwrap_or_else(|| fallback.to_string())
    elements.iter().find(|el| el.contains(color)).map_or(fallback.to_string(), |el| el.to_string()) 
}
fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    // print_elements(&colors);

    // let mut colors_iter = colors.iter();
    // shortened_string(&mut colors);

    // println!("After shortening: {:?}", colors);

    // let uppercase = to_uppercase(&colors);
    // println!("Uppercase: {:?}", uppercase);

    // let mut transform_to: Vec<String> = vec![];
    // move_elements(colors, &mut transform_to);
    // println!("Transformed elements: {:?}", transform_to);

    // let colors_exploded = explode(&colors);
    // println!("Exploded elements: {:?}", colors_exploded);

    let color_to_find = "999";
    let default_color = "black";
    let found_color = find_color_or_default(&colors, color_to_find, default_color);
    println!("Found color: {}", found_color);

}
