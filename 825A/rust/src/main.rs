fn main() {
    use std::io;
    use std::io::prelude::*;

     let stdin = io::stdin();
     let mut lc = 1;
     let mut str_len = 0;
     for line in stdin.lock().lines() {
     	match lc  {
     		1 => {
     			str_len = line.unwrap().parse::<usize>().unwrap();
     		},
     		2 => {
     			let s = line.unwrap();
     			// println!("{:?}", s);
     			let mut num = 0;
     			let mut chars = s.chars();
     			let mut prev_ch = '1';
     			for ch in chars {
     				match ch {
	     				'1' => num += 1,
	     				'0' => { print!("{}", num); num = 0; }, 
	     				_ => panic!("SOS"),
     				}
     				prev_ch = ch;
     			}
     			if num > 0 {
	     			println!("{}", num);
     			}
     			if prev_ch == '0' {
     				print!("0");
     			}
     		}
     		_ => {
     			panic!("SOS");
     		}
     	}
     	lc += 1;
     }
}
