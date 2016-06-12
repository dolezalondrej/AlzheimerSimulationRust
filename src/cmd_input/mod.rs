	use std::io;
	use std::io::Write;
	use std::str::FromStr;
	use std::fmt::Debug;
	use std::option::Option;

	///Reads & returns user input.
	pub fn get_from_user<T>(message: &str, desc: &str) -> Option<T> where T: FromStr, <T as FromStr>::Err:Debug{
		//Print information about desired input to the user
		println!("{}", message);
		print!("{}", desc);
		io::stdout().flush().unwrap();

		//Read raw input
		let mut buffer :String = String::new();
		io::stdin().read_line(&mut buffer).unwrap();

		//Parse & return
			match buffer.trim().parse::<T>(){
				Ok(e) => Option::Some(e),
				Err(_) => Option::None
			}
	}

	pub fn verify<T, F: Fn(T) -> bool>(element: T, verification:F) -> bool{
		verification(element)
	}