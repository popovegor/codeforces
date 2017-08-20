fn main() {
	use std::io;
	use std::io::prelude::*;
	
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	if let Some(Ok(line)) = lines.nth(0) {
		let mut m = line.split(" ");
		if let (Ok(n), Ok(k)) = (m.next().unwrap().parse::<usize>(), m.next().unwrap().parse::<usize>()) { 
			let d = ((n as f64 - 1f64)/(k as f64)).floor() as usize;
			match n - (d * k + 1) {
				0 => println!("{:?}", d * 2),
				1 => println!("{:?}", d * 2 + 1),
				2...100000 => println!("{:?}", d * 2 + 2),
				_ => panic!("!"),
			} 
			for i in 2..k+1 {
				println!("{:?} {:?}", 1, i);	
			}
			for i in k+1..n+1 {
				println!("{:?} {:?}", i-k, i);
			}
		}
	}
}
