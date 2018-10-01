use std::collections::HashMap;
pub enum FeedVal {
    map(HashMap<String, FeedVal>),
    val(String),
}
pub fn dec_to_ascii(dec: u8, hashmap: &HashMap<u8, char>) -> char {
    let char_or_none = hashmap.get(&dec);
    println!("{}", dec);
    //if dec == Ok(0){ // 10 is ASCII for EOF
    //return '`';
    //}
    match char_or_none {
        Some(char) => char_or_none.unwrap().to_owned(),
        _ => '_',
    }
}
pub fn data_to_FeedVal(is_tag_name: bool, data: String) -> (bool, String, bool) {
    ///(pop?,data,is_tag_name?)
    let data_char_vec: Vec<char> = data.chars().collect();
    let slash_char: char = data_char_vec[0];
    let tag_name: String = data_char_vec[1..].iter().collect();
    match (slash_char, is_tag_name) {
        ('/', true) => (true, data, true),  // pop  tag name
        (_, true) => (false, data, true),   // push tag name
        (_, false) => (false, data, false), // push tag data
    }
}
fn main() {
    println!("{:?}", data_to_FeedVal(true, "/item".to_string()));
    println!("{:?}", data_to_FeedVal(false, "hello".to_string()));
    println!("{:?}", data_to_FeedVal(true, "item".to_string()));
}
