fn main() {
    
    use std::io;
    use std::io::prelude::*;
    use std::cmp;
    use std::collections::HashMap;
    use std::time::Instant;
    use std::ffi::OsString;


    let time1 = Instant::now();

    let mut ss : Vec<Vec<u8>> = Vec::new();

    let stdin = io::stdin();
    let mut line_counter = 0;
    let mut n = 0usize;
    let mut sp : HashMap<usize, (usize, usize)> = HashMap::new();
    let mut pos_max = 0usize;
    for line in stdin.lock().lines() {
    	if let Ok(ref l) = line {
	    	line_counter += 1;
	    	match line_counter {
	    		1 => n = l.parse::<usize>().unwrap(), 
	    		_ => {
	    			// println!("{:?}", l);
	    			let time3 = Instant::now();
	    			let mut iter = l.split_whitespace();
	    			// println!("split - {:?}", time3.elapsed());
	    			let mut substr = iter.next().unwrap();
	    			let k = iter.next().unwrap().parse::<usize>().unwrap();
    				let s_bytes = String::from(substr).into_bytes(); 
					let substr_len = s_bytes.len();
	    			let mut n = ss.push(s_bytes);
    				let time2 = Instant::now();
    				let s_num = line_counter - 1;
	    			while let Some(position) = iter.next() {
	    				if let Ok(pos) = position.parse::<usize>() {
	    					pos_max = cmp::max(pos_max, pos+substr_len-1);
	    					let time4 = Instant::now();
	    					let mut m = sp.entry(pos).or_insert((s_num, substr_len));
	    					if m.1 < substr_len {
	    						*m = (s_num, substr_len);
	    					}
		    				// s.insert(pos, substr.to_string());
	    				}
	    			}
	    			// println!("{:?}", time2.elapsed());
	    		},
	    	}
    	}
    }
    // println!("{:?}", ss);
    // println!("{:?}", sp);
    let stdout = io::stdout();
	let mut handle = stdout.lock();
	let mut i = 1;
 	let mut _i = 0;
 	let mut _s_ = String::new();
 	// let mut _s_chars = _s_.chars();
 	let mut _s_len  = 0;
	while i <= pos_max {
		if let Some((s_num, s_len)) = sp.remove(&i) {
			if _s_len < s_len + _i {
				let ss_bytes = ss.get(s_num-1).unwrap()[_s_len - cmp::min(_i, _s_len)..s_len].to_vec();
				let ss_str = String::from_utf8(ss_bytes);
				print!("{}",  ss_str.unwrap());
				_s_len = s_len;
				_i = 0;
			}
		}
		if _i >= _s_len  {
			print!("a");
		}
		i += 1;
		_i += 1;
	}

   println!("");
}
