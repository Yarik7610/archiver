# Archiver
This is a small rust command line programm that converts your txt file to less binary file in size. It uses opened Huffmans coding algorithm.
## How to use
1. Clone this repo
2. Add some txt file in the root of project
3. Write in terminal `cargo run yourfilename.txt`
4. After that you will see the old and the new size. Also if there will be 2 files: huffmans_encoded.bin and huffmans_decoded.txt. The first file makes a binary coded representation of your text by Huffmans algorithm, the second file parses a binary file and converts it back to your initial text
### My comment
It was pretty interesing task to do, escpecially on Rust. It is not so hard to understand it, so try it out)
