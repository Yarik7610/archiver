use crate::huffmans_node::Node;
use std::{
    collections::{BinaryHeap, HashMap},
    error::Error,
    fs::File,
    io::Write,
};

pub fn map_chars(file_contents: &String) -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();
    for line in file_contents.lines() {
        for ch in line.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }
    }
    map
}

pub fn build_min_heap(map: &HashMap<char, u32>) -> BinaryHeap<Node> {
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    for (ch, freq) in map {
        let node = Node {
            ch: Some(*ch),
            freq: *freq,
            left: None,
            right: None,
        };
        heap.push(node);
    }
    heap
}

pub fn create_huffman_tree(mut heap: BinaryHeap<Node>) -> Option<Node> {
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let new = Node {
            ch: None,
            freq: left.freq + right.freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };
        heap.push(new);
    }
    //return root
    heap.pop()
}

pub fn get_codes(root: &Option<Node>) -> HashMap<char, String> {
    let mut codes: HashMap<char, String> = HashMap::new();
    if let Some(root) = root {
        let mut stack: Vec<(&Node, String)> = vec![(root, String::from(""))];
        while let Some((cur, code)) = stack.pop() {
            if cur.left.is_none() && cur.right.is_none() {
                codes.insert(cur.ch.unwrap(), code);
                continue;
            }
            let left = &cur.left;
            let right = &cur.right;
            if let Some(right) = right {
                stack.push((right, code.clone() + "1"));
            }
            if let Some(left) = left {
                stack.push((left, code.clone() + "0"));
            }
        }
    }
    codes
}

pub fn encode_text(
    codes: &HashMap<char, String>,
    file_contents: &String,
) -> Result<String, Box<dyn Error>> {
    let mut output: Vec<u8> = vec![];
    //8 bits will be stored in 1 byte, as one char, but the will mean more than 1 char mostly
    let mut current_byte: u8 = 0;
    let mut num_bits = 0;
    for line in file_contents.lines() {
        for ch in line.chars() {
            let char_code = codes.get(&ch).unwrap();
            for bit in char_code.chars() {
                //we dont process 0 because it is automatically put in byte
                if bit == '1' {
                    // |= means make a logical or of current byte with right side 1 << (7 - num_bits), and after that assign the result to current_byte
                    current_byte |= 1 << (7 - num_bits);
                }
                num_bits += 1;
                if num_bits == 8 {
                    output.push(current_byte);
                    current_byte = 0;
                    num_bits = 0;
                }
            }
        }
    }
    if num_bits > 0 {
        output.push(current_byte);
    }
    let mut f = File::create("huffmans_output.bin")?;
    f.write_all(&output)?;
    Ok(output.len().to_string())
}
