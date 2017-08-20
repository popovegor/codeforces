fn main() {
    use std::io;
    use std::io::prelude::*;

    let stdin = io::stdin();
    let mut counter = 0;
    let mut g_count = 0;
    let mut t1_count = 0; 
    let mut t2_count = 0;
	for line in stdin.lock().lines() {
		// println!("{:?}", line);
		if let Ok(l) = line {
			counter += 1;
			match counter {
				1 => {
					let mut l1 = l.split(" ");
					if let (Ok(g), Ok(t1), Ok(t2)) = (l1.nth(0).unwrap().parse::<i32>()
						, l1.nth(0).unwrap().parse::<i32>()
						, l1.nth(0).unwrap().parse::<i32>()) {
						g_count = g;
						t1_count = t1;
						t2_count = t2; 
					}
					// println!("{:?} {:?} {:?}", g_count, t1_count, t2_count);

				},
				2 => {
					 let mut t1_free = t1_count;
					 let mut t2_free = t2_count;
					 let mut t2_one_free = t2_count * 2;
					 let mut total_denies = 0;
					 let mut l2 = l.split(" ");
					 while let Some(g) = l2.next() {
					 	match g {
					 		"1" => {
					 			// println!("{:?}", 1);
					 			if t1_free > 0 {
					 				t1_free -= 1;
					 			} else if t2_free > 0 {
					 				t2_free -= 1;
					 				t2_one_free -= 1;
					 			} else if t2_one_free > 0 {
					 				t2_one_free -= 1;
					 				// if t2_one_free % 2 == 0 {
					 				// 	t2_free -= 1;
					 				// }
					 			} else {
					 				total_denies += 1;
					 			}
					 			
					 		},
					 		_ => {
					 			// println!("{:?}", 2);
					 			if t2_free > 0 {
					 				t2_free -= 1;
					 				t2_one_free -= 2;
					 			} else {
					 				total_denies +=2;
					 			}
					 		}, 
					 	}
					 	// println!("{:?} {:?} {:?}", t1_free, t2_free, t2_one_free);
					 }
					 println!("{:?}", total_denies);
				},
				_ => panic!("!"),
			}
		}
	}

}
