extern crate rand;
use rand::Rng;

fn main() {
	let n = 100000;
	let mut v:Vec<i32> = get_ran_vec(n);

	println!("welcome to insertion sort");
	println!("the list before is : {:?} " , v);
	println!("SORTING ....");
	println!("{:?}" , insertion_sort(&mut v));
	println!("SORTED");
}

fn get_ran_vec( n:usize ) -> Vec<i32> {
	let mut v: Vec<i32> = Vec::with_capacity( n );
	let mut rng = rand::thread_rng();
	for _ in 0..n {
		v.push(rng.gen::<i32>());
	}
	v
}

fn insertion_sort( v:&mut Vec<i32> ) -> &Vec<i32> {
    // go over all the elements
		for i in 1..v.len() {
			let curr = v[i];
			for j in (0..i).rev() {
				// println!("{:?}", v);
				if curr < v[j] {
					v[j+1] = v[j];
					if j == 0 {
						v[0] = curr;
					}
				}
				else {
					v[j+1] = curr;
					break;
				}
			}

		}
		v
}
