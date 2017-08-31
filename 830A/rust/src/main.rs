fn main() {
    use std::io::stdin;
    use std::io::prelude::*;
    use std::collections::{HashMap, HashSet};
    use std::i32;

    let stdin = stdin();
    let mut lc = 0;
    let mut men : HashSet<i32> = HashSet::new();
    let mut keys : HashSet<i32> = HashSet::new();
    let mut office : i32 = 0; 
    while let Some(Ok(l)) = stdin.lock().lines().next() {
    	lc += 1;
    	match lc {
    		1 => {
    			let mut m = l.split_whitespace();
    			office = m.nth(2).unwrap().parse::<i32>().unwrap(); 
    			// println!("{:?}", office);
    		},
    		2 => {
    			for man in l.split_whitespace() {
    				let man = man.parse::<i32>().unwrap(); 
    				men.insert(man); 
    			}
    		},
    		3 => {
    			for key in l.split_whitespace() {
    				let key = key.parse::<i32>().unwrap();
    				keys.insert(key);
    				// println!("{:?}", man);
    				
    			}
    		},
    		_ => panic!("SOS"),
    	}
    }
    // men.sort();
    let mut max_dist = 0;
    let mut men_sorted = men.into_iter().collect::<Vec<i32>>();
    men_sorted.sort_by(|a, b| a.cmp(b));
    // println!("{:?}", men_sorted.iter().collect::<Vec<&i32>>());
    let mut keys_sorted = keys.into_iter().collect::<Vec<i32>>();
    keys_sorted.sort_by(|a, b| a.cmp(b));
    // println!("{:?}", keys_sorted.iter().collect::<Vec<&i32>>());

    let mut men_office = Vec::new();
    for man in men_sorted.iter() {
    	men_office.push( ((*man - office).abs(), *man) );
    }

    men_office.sort_by(|x, y| y.0.cmp(&x.0));
    // println!("{:?}", men_office);


	// while men_office.len() > 0 {
		let mut min_dist = i32::MAX;
		for i in 0..keys_sorted.len() - men_sorted.len() + 1 {
			let mut max_key : i32 = 0;
			let mut max_dist = 0;
			let mut max_man : i32 = 0;
			let mut max_key_idx : usize = 0;
			let mut max_man_idx : usize = 0;
			// let mut min_dist : i32 = i32::MAX;
			// let man = men_office[i].1;
			for j in 0..men_sorted.len() {
				let man = men_sorted[j];
				let key = keys_sorted[i + j];
				let dist = (key - man).abs() + (key - office).abs();
				if max_dist < dist {
					max_dist = dist;
					max_key = key;
					max_man = man;
					max_man_idx = i;
					max_key_idx = j;
				}
			}
			min_dist = std::cmp::min(min_dist, max_dist);
			// println!("{:?} -> {:?} -> {:?} = {:?}", max_man, max_key, office, max_dist);
			// keys_sorted.remove(min_key_idx);
		// }
		// men_office.remove(min_man_idx);
		// keys_sorted.remove(min_key_idx);

	}
	
	// for i in 0..men_sorted.len() {
	// 	let key = keys_sorted[(min_i + i) % keys_sorted.len()];
	// 	let man = men_sorted[i];
	// 	max_dist = std::cmp::max(max_dist, (key - man).abs() + (key - office).abs());
	// }
	// let key = keys_sorted[];
	// let man = men_sorted.last().unwrap();
	

    println!("{:?}", min_dist);
}
