mod xml_parse_lib;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use xml_parse_lib::*;

fn main() {
    let mut ascii_map: HashMap<u8, char> = HashMap::new();
    let mut tag_buff: Vec<char> = Vec::new();
    ascii_map.insert(32, ' ');
    ascii_map.insert(33, '!');
    ascii_map.insert(34, '"');
    ascii_map.insert(35, '#');
    ascii_map.insert(36, '$');
    ascii_map.insert(37, '%');
    ascii_map.insert(38, '&');
    ascii_map.insert(39, '\'');
    ascii_map.insert(40, '(');
    ascii_map.insert(41, ')');
    ascii_map.insert(42, '*');
    ascii_map.insert(43, '+');
    ascii_map.insert(44, ',');
    ascii_map.insert(45, '-');
    ascii_map.insert(46, '.');
    ascii_map.insert(47, '/');
    ascii_map.insert(48, '0');
    ascii_map.insert(49, '1');
    ascii_map.insert(50, '2');
    ascii_map.insert(51, '3');
    ascii_map.insert(52, '4');
    ascii_map.insert(53, '5');
    ascii_map.insert(54, '6');
    ascii_map.insert(55, '7');
    ascii_map.insert(56, '8');
    ascii_map.insert(57, '9');
    ascii_map.insert(58, ':');
    ascii_map.insert(59, ';');
    ascii_map.insert(60, '<');
    ascii_map.insert(61, '=');
    ascii_map.insert(62, '>');
    ascii_map.insert(63, '?');
    ascii_map.insert(64, '@');
    ascii_map.insert(65, 'A');
    ascii_map.insert(66, 'B');
    ascii_map.insert(67, 'C');
    ascii_map.insert(68, 'D');
    ascii_map.insert(69, 'E');
    ascii_map.insert(70, 'F');
    ascii_map.insert(71, 'G');
    ascii_map.insert(72, 'H');
    ascii_map.insert(73, 'I');
    ascii_map.insert(74, 'J');
    ascii_map.insert(75, 'K');
    ascii_map.insert(76, 'L');
    ascii_map.insert(77, 'M');
    ascii_map.insert(78, 'N');
    ascii_map.insert(79, 'O');
    ascii_map.insert(80, 'P');
    ascii_map.insert(81, 'Q');
    ascii_map.insert(82, 'R');
    ascii_map.insert(83, 'S');
    ascii_map.insert(84, 'T');
    ascii_map.insert(85, 'U');
    ascii_map.insert(86, 'V');
    ascii_map.insert(87, 'W');
    ascii_map.insert(88, 'X');
    ascii_map.insert(89, 'Y');
    ascii_map.insert(90, 'Z');
    ascii_map.insert(91, '[');
    ascii_map.insert(92, '\'');
    ascii_map.insert(93, ']');
    ascii_map.insert(94, '^');
    ascii_map.insert(95, '_');
    ascii_map.insert(96, '`');
    ascii_map.insert(97, 'a');
    ascii_map.insert(98, 'b');
    ascii_map.insert(99, 'c');
    ascii_map.insert(100, 'd');
    ascii_map.insert(101, 'e');
    ascii_map.insert(102, 'f');
    ascii_map.insert(103, 'g');
    ascii_map.insert(104, 'h');
    ascii_map.insert(105, 'i');
    ascii_map.insert(106, 'j');
    ascii_map.insert(107, 'k');
    ascii_map.insert(108, 'l');
    ascii_map.insert(109, 'm');
    ascii_map.insert(110, 'n');
    ascii_map.insert(111, 'o');
    ascii_map.insert(112, 'p');
    ascii_map.insert(113, 'q');
    ascii_map.insert(114, 'r');
    ascii_map.insert(115, 's');
    ascii_map.insert(116, 't');
    ascii_map.insert(117, 'u');
    ascii_map.insert(118, 'v');
    ascii_map.insert(119, 'w');
    ascii_map.insert(120, 'x');
    ascii_map.insert(121, 'y');
    ascii_map.insert(122, 'z');
    ascii_map.insert(123, '{');
    ascii_map.insert(124, '|');
    ascii_map.insert(125, '}');
    ascii_map.insert(126, '~');
    let mut reader = BufReader::new(
        File::open("/home/alexbrown/Music/.xml/tester_fmtted.xml").expect("open failed"),
    );
    let mut buffer = [0; 16];
    let mut flag_pop_name: bool = false;
    let mut rec_tag_name: bool = false;
    let mut rec_tag_data: bool = false;
    let mut tag_collector_string: String;
    let mut bytes_read: usize;
    let mut curr_byte = 1;
    let mut last_read = false;
    let mut tag_stack: Vec<String> = Vec::new();
    'read_file: loop {
        curr_byte = 0;
        bytes_read = reader.read(&mut buffer[..]).unwrap();
        let chunk: Vec<char> = buffer
            .iter()
            .map(|byte| dec_to_ascii(*byte, &ascii_map))
            .collect();
        if bytes_read < 15{
            last_read = true;
        }
        'read_chunk: for byte in chunk.iter() {
            // BEGIN CRAPPY PARSING
            if *byte == '<'{
                if rec_tag_data{
                    tag_collector_string = tag_buff.clone().into_iter().collect();
                    rec_tag_data = false;
                    rec_tag_name = true;
                }
            } else if *byte == '/'{
                if rec_tag_name{
                    flag_pop_name = true;
                }
            } else if *byte == '>'{
                if flag_pop_name{
                    tag_collector_string = tag_buff.clone().into_iter().collect();
                    if tag_stack.contains(&tag_collector_string){
                        rec_tag_name = false;
                        rec_tag_data = true;
                    }
                }else{
                    tag_collector_string = tag_buff.clone().into_iter().collect();
                    println!("pushing: {}",tag_collector_string);
                    tag_stack.push(tag_collector_string);
                    rec_tag_name = false;
                    rec_tag_data = true;
                }
            } else {
                tag_buff.push(*byte);
            }
            // END CRAPPY PARSING
            curr_byte +=1;
            if curr_byte == bytes_read && last_read{
                break 'read_file;
            }
        }
    }
}
