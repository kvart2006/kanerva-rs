///
/// Import 'array_float_constants.rs'.
/// The file was generated with values defined in 'build.rs'.
///
#[allow(missing_docs)]
include!(concat!(env!("OUT_DIR"), "/array_constants.rs"));

extern crate rand;
//
use rand::distributions::{Distribution, Range};
///
use array::rand::distributions::Bernoulli;
///
use array::rand::{thread_rng, Rng};
///
use std::ops::{BitXor, BitAnd, BitOr};
///
use std::collections::HashMap;
///
use std::fmt;
///
/// Define an array of bool of dimension 'DIM1'.
///
#[derive(Copy, Clone)]
pub struct BoolArray( [bool; DIM] );
///
///
///
impl fmt::Debug for BoolArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BA.{{len() = {}}}", self.0.len())
    }
}
///
///
///
impl Default for BoolArray {
	fn default() -> BoolArray { BoolArray::new() }
}
///
///
///
impl PartialEq for BoolArray {

	fn eq(&self, other: &BoolArray) -> bool {
		let mut ret: bool = false;
		
		for it in self.0.iter().zip(other.0.iter()) { 
			let (ai, bi) = it;
			if *bi == *ai { ret = true } else { ret = false }
		}
		ret
    }
}
///
/// Implementation for 'BooleanArray'.
///
impl BoolArray {
	///
	///
	///
	pub fn new() -> Self {
		let d = Bernoulli::new(0.5);
		let mut a: [bool; DIM] = [false; DIM];

		let v: Vec<bool> = thread_rng().sample_iter(&d).take(DIM).collect::<Vec<bool>>();

        a.copy_from_slice(&v);

		BoolArray(a)
	}
	///
	///
	///
	pub fn new_from(a: &[bool; DIM]) -> Self { BoolArray(*a) }
	///
	/// Create a BoolArray from slice '&[bool]'. 
	///
	pub fn new_from_vec(v: &[bool]) -> Self { 
		let mut a: [bool; DIM] = [false; DIM];
		a.copy_from_slice(&v[0..v.len()]);
		BoolArray(a) 
	}
    ///
    /// Construct a new 'BoolArray' of dimension 'DIM' with 'CARD' ON values. 
    ///
    pub fn new_card(card: usize) -> Self {
    	let mut r;
    	let mut a: [bool; DIM] = [false; DIM];
    	let mut v: Vec<usize> = vec![0; card];
    	let mut indices: HashMap<usize, usize> = HashMap::with_capacity(card);

    	let mut rng = thread_rng();

        let inrange = Range::new(0, DIM);

        while indices.len() < card {
            r = inrange.sample(&mut rng);
            indices.insert(r, r);
        }

        r = Default::default();
        for val in indices.values() {
            v[r] = *val;
            r += 1;
        }
	
        for i in 0..card { a[v[i]] = true }

        BoolArray(a)
    }
    ///
    ///
    ///
    pub fn len(&self) -> usize { self.0.len() }  
    ///
    ///
    ///
    pub fn get(&self, i: usize) -> bool {
    	assert!(i<self.len());
        self.0[i]
    }
    ///
    ///
    ///
    pub fn set(&mut self, i: usize, value: bool) {
    	assert!(i<self.len());
        self.0[i] = value;
        assert_eq!(self.0[i], value);
    }
    ///
    ///
    ///
    pub fn count_zeros(&self) -> usize {
    	let BoolArray(a) = self;
    	let mut zeros: usize = 0;
    	for x in a.iter() { if x == &false { zeros += 1; } }; 
    	zeros
    }
    ///
    ///
    ///
    pub fn count_ones(&self) -> usize {
    	let BoolArray(a) = self;
    	let mut ones: usize = 0;
    	for x in a.iter() { if x == &true { ones += 1; } }; 
    	ones
    }
    ///
    /// Print content. 
    ///
    pub fn show<'a, T: Into<Option<&'a str>>>(&self, name: T) {
        let BoolArray(a) = self;
        if let Some(name) = name.into() { 
        	print!("BA({})[", name); 
        } else {
            print!("BA["); 
        }
        for x in a.iter() { if x == &false { print!("0") } else { print!("1") } }; 
        print!("]"); 
    }


}
///
/// The 'bitxor' operator for 'BooleanArray' of dimension 'DIM1'.
/// A	B	 A XOR B
/// 0	0		0
/// 0	1		1
/// 1	0		1
/// 1	1		0
///
impl BitXor for BoolArray {
    type Output = Self;

    fn bitxor(self, BoolArray(rhs): Self) -> Self {
        let BoolArray(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        let mut i: usize = 0;
        let mut a: [bool; DIM] = [false; DIM];

        let lv = lhs.iter()
        .zip(rhs.iter())
        .map(|(x, y)| (*x || *y) && !(*x && *y))
        .collect::<Vec<bool>>();

        for x in a.iter_mut() { *x = lv[i]; i += 1; }

        BoolArray(a)
    }
}
///
/// The 'bitand' operator for 'BooleanArray' of dimension 'DIM1'.
/// A	B	 A AND B
/// 0	0		0
/// 0	1		0
/// 1	0		0
/// 1	1		1
///
impl BitAnd for BoolArray {
    type Output = Self;

    fn bitand(self, BoolArray(rhs): Self) -> Self {
        let BoolArray(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        let mut i: usize = 0;
        let mut a: [bool; DIM] = [false; DIM];

        let lv = lhs.iter()
        .zip(rhs.iter())
        .map(|(x, y)| *x && *y)
        .collect::<Vec<bool>>();

        for x in a.iter_mut() { *x = lv[i]; i += 1; }

        BoolArray(a)
    }
}
///
/// The 'bitor' operator for 'BooleanArray' of dimension 'DIM1'.
/// A	B	  A OR B
/// 0	0		0
/// 0	1		1
/// 1	0		1
/// 1	1		1
///
impl BitOr for BoolArray {
    type Output = Self;

    fn bitor(self, BoolArray(rhs): Self) -> Self {
        let BoolArray(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        let mut i: usize = 0;
        let mut a: [bool; DIM] = [false; DIM];

        let lv = lhs.iter()
        .zip(rhs.iter())
        .map(|(x, y)| *x || *y)
        .collect::<Vec<bool>>();

        for x in a.iter_mut() { *x = lv[i]; i += 1; }

        BoolArray(a)
    }
}

