use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

///Parses values implemented std::str::FromStr trait from a given file.
pub fn numbers_from_file<T: FromStr>(file: &mut File)-> Vec<T>{
	//Read the whole file into memory
	let mut buffer: String = String::new();
	file.read_to_string(&mut buffer).unwrap();

	//Parse the values
	let mut vec: Vec<T> = Vec::new();
	
	for line in buffer.lines_any(){
		let parsed_value: Option<T> = line.parse::<T>().ok();
		match parsed_value {
			Some(n) => vec.push(n),
			None => println!("Failed to parse value: {}", line),
		}
	}
	vec
}

pub fn write_vec_to_file<T: ToString>(file: &mut File, vec: &Vec<T>) {
	let mut buffer : String = String::new();

	for token in vec.into_iter(){
		buffer.push_str(&token.to_string());
		buffer.push_str(&"\n");
	}

	file.write(&buffer.into_bytes()).unwrap();
}

