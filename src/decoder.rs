use crate::huffmans_node::Node;
use std::{error::Error, fs::File, io::Read};

pub fn decode(root: &Option<Node>) -> Result<String, Box<dyn Error>> {
    let mut file = File::open("huffmans_output.bin")?;
    let mut buf: Vec<u8> = vec![];
    //decimal bytes representation
    file.read_to_end(&mut buf)?;
    let decoded_binary_string = decimal_bytes_to_binary_string(buf);
    let mut decoded_string = String::from("");
    if let Some(root) = root {
        let mut cur = root;
        for digit in decoded_binary_string.chars() {
            if cur.left.is_none() && cur.right.is_none() {
                decoded_string.push(cur.ch.unwrap());
                cur = root;
            }
            if digit == '0' {
                cur = cur.left.as_deref().unwrap();
            } else if digit == '1' {
                cur = cur.right.as_deref().unwrap();
            }
        }
        //if loop ended but cur is a leave we should add it to decoded string
        if cur.left.is_none() && cur.right.is_none() {
            decoded_string.push(cur.ch.unwrap());
        }
    }
    Ok(decoded_string)
}

fn decimal_bytes_to_binary_string(v: Vec<u8>) -> String {
    //b: means binary format
    //:0 means if the length is less than 8 it will be filled with 0
    //:8 means length will be 8 symbols
    v.iter().map(|byte| format!("{:08b}", byte)).collect()
}
