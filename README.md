# Archiver
This is a small rust command line program that converts your txt file to less binary file in size. It uses source opened Huffman coding algorithm.
## How to use
0. Install rust
1. Clone this repo
2. Add some txt file in the root of project or use default test.txt
3. Write in terminal `cargo run yourfilename.txt` or `cargo run test.txt`
4. After that you will see the old and the new sizes. Also there will be 2 files: huffmans_encoded.bin and huffmans_decoded.txt. The first file makes a binary coded representation of your text by using Huffman coding, the second file parses a binary file and converts it back to your initial text
## My comment
It was pretty interesing task to do, escpecially on Rust. It is not so hard to understand it, so try it out)
