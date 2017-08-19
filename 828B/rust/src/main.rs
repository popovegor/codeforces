fn main() {
	use std::io;
	use std::io::prelude::*;
	use std::cmp;

	let stdin = io::stdin();
	let mut counter = 0;
	let mut w = 0;
	let mut h = 0;
	let (mut left, mut right, mut bottom, mut top) = (100,1,1,100);
	let mut black_counter = 0;
	let mut white_counter = 0;
	for line in stdin.lock().lines() {
		
	   	if counter == 0 {
	   		let l = line.unwrap();
			let mut l1 = l.split(" ");
			h = l1.next().unwrap().parse::<i32>().unwrap();
			w = l1.next().unwrap().parse::<i32>().unwrap();
	   		// println!("w{:?} - h{:?}", w, h);
	   	} else {
	   		let l = line.unwrap_or(String::default());
	   		let mut chars = l.chars();
	   		let mut i = 0;
	   		while let Some(ch) = chars.next() {
	   			i += 1;
	   			if ch == 'B' {
	   				black_counter += 1;
	   				left = cmp::min(left, i);
	   				top = cmp::min(counter, top);
	   				right = cmp::max(right, i);
	   				bottom = cmp::max(bottom, counter);
	   			} else {
	   				white_counter += 1;
	   			}
	   			// println!("c{:?} i{:?} {:?}", counter, i, ch);
	   		}
	   		
	   		// println!("{:?}", l);
	   	}
	   	counter += 1;
	}

	if black_counter == 0 && white_counter > 0 {
		println!("{:?}", 1);
	} else {
		let square_side_len = cmp::max((right - left + 1), (bottom - top + 1));
		let black_needed = square_side_len * square_side_len - black_counter;
		if black_needed < 0 || (black_needed + black_counter) > w*h || square_side_len > w || square_side_len > h {
			println!("{:?}", -1);
		} else {
			println!("{:?}", black_needed);
		}
	}
	// println!("{:?} {:?} {:?} {:?}", right, left, bottom, top);
}
