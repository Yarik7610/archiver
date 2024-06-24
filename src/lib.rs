mod decoder;
mod encoder;
mod huffmans_node;
use std::{error::Error, fs::File, io::Read, process};

pub fn run(filename: &String) -> Result<(), Box<dyn Error>> {
    let (input_contents, input_size) = get_file_data(&filename).unwrap_or_else(|err| {
        eprintln!("Input file problem: {err}");
        process::exit(1);
    });
    println!("Current file size in bytes: {}", input_size);
    let map = encoder::map_chars(&input_contents);
    let heap = encoder::build_min_heap(&map);
    let root = encoder::create_huffman_tree(heap);
    let codes = encoder::get_codes(&root);
    let (encoded_size, extra_bits) = encoder::encode_text(&codes, &input_contents)?;
    println!("New file size in bytes: {}", encoded_size);
    let decoded_text = decoder::decode(&root, extra_bits)?;
    decoder::write_to_file(decoded_text)?;
    Ok(())
}

pub fn get_filename(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();
    match args.next() {
        Some(filename) => Ok(filename),
        None => Err("Didn't get filename. Please, provide filename in command line."),
    }
}
fn get_file_data(filename: &str) -> Result<(String, u64), Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut buffer = String::from("");
    file.read_to_string(&mut buffer)?;
    Ok((buffer, file.metadata().unwrap().len()))
}
