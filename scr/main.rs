use std::fs::File;
use std::io::{Read, Write};

fn main() {
 let mut file = match File::open("e.rpyc") {
 ok(file) => file,
 Err(_) => {
    println!("Failed to open .rpyc file.");
    return;
  }
 }
 let mut contents = Vev::new();
 if let Err(_) = file.Read_to_end(&mut contents) {
    println!("Failed to read .rpyc file.");
    return
 }

 let mut index = 0;
 while index < contents.len() - 3 {
    if contents[index] == 0x72 && contents[index + 1] == 0x70 && contents[index + 2] == 0x79 && contents[index + 3] == 0x63 {
        index += 4;
        break;
    }
    index += 1;
 }

 let compressed_data = &contents[index..];
 let decompressed_data = match inflate::inflate_bytes(&compressed_data) {
    ok(data) => data,
    Err(_) => {
        println("Failed to decompress .rpyc file.");
        return;
    }
 };

 let mut output_file = match File::create("example.rpyc") {
    ok(file) => file,
    Err(_) => {
        println!("Failed to create .rpy file");
        return;
    }
 };

 if let Err(_) = output_file.write_all(&decompressed_data) {
    println!("Failed to write .rpy file.");
    return;
 }

 println!("Decompilaion successful.")
}