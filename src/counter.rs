///
/// Import 'constants.rs'.
/// The file was generated with values defined in 'build.rs'.
///
#[allow(missing_docs)]
include!(concat!(env!("OUT_DIR"), "/dimensions.rs"));
///
use std::mem;
///
use std::fmt;
///
use bittable::BitTable;
///
/// Number of bits: in a `usize` (`64` in a 64 bits target).
///
#[inline(always)]
const fn bits() -> usize { mem::size_of::<usize>() * 8 }
/// Compute the position for `DIM*ADD_DIM` elements where
/// `index` in `0..ADD_DIM`
/// `i` in `0..DIM*bits()`
#[inline(always)]
fn pos(index: usize, i: usize) -> usize {
    let pos = (DIM*bits())*index+i;
    pos
}
///
/// Counter fpr hard locations.
///
#[derive(Clone, Debug)]
pub struct Counter (Vec<i16>);
///
///
///
/*
impl fmt::Debug for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "C.{{len() = {}}}", self.0.len())
    }
}*/
///
///
///
impl Counter {

	///
	/// 
	///
	pub fn new() -> Self {
		let v: Vec<i16>=vec![0; DIM*bits()*ADD_DIM]; 
		Counter(v)
	}
	///
	///
	///
	pub fn len(&self) -> usize { self.0.len() }
	/// Get counter value for address index, at position i.
	/// `index` in `ADD_DIM`
	/// `i` in `DIM`
	pub fn get(&self, index: usize, i: usize) -> i16 {
		self.0[pos(index,i)]
	} 
	/// Set counter value for address index, at position i.
	/// `index` is in `0..ADD_DIM`
	/// `i` in `0..DIM`
	pub fn set(&mut self, index: usize, i: usize, b: bool) {
		if b { self.0[pos(index,i)] += 1; } else { self.0[pos(index,i)] -= 1; }
	} 
	///
	/// Update the counters at address `index` using input `b`.
	///
	pub fn input(&mut self, index: usize, b: &BitTable) {
		for i in 0..DIM*bits() { 
			if b.get(i) { self.set(index, i, true); } 
			else { self.set(index, i, false); } 
		};
	}
	///
	///
	///
	pub fn get_counters(&mut self, index: usize) -> Vec<i16> {
		let mut res: Vec<i16> = vec![0; DIM*bits()];
		for i in pos(index,0)..(pos(index,0)+DIM*bits()) { res[i-pos(index,0)] = self.0[i]; }
		res
	}
	///
	/// Print the content. 
	///
	pub fn show(& self) {
		print!("\nC[\n");
		for index in 0..ADD_DIM { 
			for i in 0..(DIM*bits()) { print!("{},", self.get(index,i)); }
    		println!();
    	}
    	print!("]\n");
	}

}

#[cfg(test)]
mod tests {

	use std::mem;
	use super::{pos, bits};
    use super::Counter;
    use super::BitTable;
    use super::{ADD_DIM, DIM};

    #[test]
    fn test_counter_size() {
        println!("size_of<Counter({},{})> = {} in B.\n", ADD_DIM, DIM*bits(), mem::size_of::<Counter>());
    	assert_eq!(24, mem::size_of::<Counter>());
    } 

     #[test]
    fn test_counter_pos() {
    	let mut v: usize;
    	let _: Counter = Counter::new();  
    	for index in 0..ADD_DIM {
			v=pos(index, 0);
			assert_eq!(v, index*(DIM*bits()));
		}
    } 

     #[test]
    fn test_counter_len() {
    	let c: Counter = Counter::new();  
    	assert_eq!(DIM*bits()*ADD_DIM, c.len());
    } 

    #[test]
    fn test_counter_get() { 
    	let mut cont: Counter = Counter::new(); 
    	let index = ADD_DIM-1;
    	let i = DIM-1;
    	cont.set(index, i, true);
    	let ci1 = cont.get(index, i);
		assert_eq!(ci1, 1);
    	cont.set(index, i, true);
    	let ci2 = cont.get(index, i);
		assert_eq!(ci2, 2);
    	cont.set(index, i, false);
    	let ci3 = cont.get(index, i);
		assert_eq!(ci3, 1);
	}

    #[test]
	fn test_counter_get_counters() {
		let mut cont: Counter = Counter::new(); 
		let mut v: Vec<i16>;
		for index in 0..ADD_DIM {
			v = cont.get_counters(index);
		}
	}

    #[test]
    fn test_counter_input() { 
    	let bt = BitTable::new_rand();
    	//bt.show();
		let mut c: Counter = Counter::new(); 
		for index in 0..ADD_DIM { c.input(index, &bt); }
		//c.show();
    }
}
