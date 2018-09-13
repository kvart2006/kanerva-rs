#[allow(missing_docs)]
include!(concat!(env!("OUT_DIR"), "/dimensions.rs"));
///
use rand::distributions::Bernoulli;
///
use rand::{thread_rng, Rng};
///
use std::mem;
///
use std::fmt;
///
use std::ptr;
///
use std::ops::{BitXor, BitAnd, BitOr};
///
use bittable::BitTable;
///
/// Number of bits: in a `usize` (`64` in a 64 bits target).
///
#[inline(always)]
const fn bits() -> usize { mem::size_of::<usize>() * 8 }
///
///
///
pub struct Vector (BitTable);
///
///
///
impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "V.{{len() = {}}}", self.0.len())
    }
}
///
///
///
impl Clone for Vector {
    #[inline]
    fn clone(&self) -> Self {
    	let Vector(lhs) = self;
        Vector(lhs.clone())
    }
}
///
///
///
impl Vector {
	///
	/// 
	///
	pub fn new() -> Self {
		Vector(BitTable::new_rand(SDM_DIM))
	}
	///
	/// 
	///
	pub fn new_from_bittable(bt: &BitTable) -> Self {
		Vector(bt.clone())
	}
	///
	/// 
	///
	pub fn new_from_slice(s: &[bool]) -> Self {
		assert_eq!(s.len(), SDM_DIM);
		let bt=BitTable::from_slice(s);
		Vector(bt)
	}
	///
	/// The `new_from_vec` method doesn't give ownership of `s`.
	/// No method can consume the value of `s`.
	/// A copy into a new `Vec<bool>` is needed.
	///
	pub fn new_from_vec(s: &Vec<bool>) -> Self {
		assert_eq!(s.len(), SDM_DIM*bits());
		let bt=BitTable::from_slice(s.as_slice());
		Vector(bt)
	}
	///
	///
	///
	pub fn as_slice(&self) -> &[usize] {
		let Vector(vb) = self;
		vb.as_slice()
	}
	///
	///
	///
	pub fn to_bittable(&self) -> &BitTable {
		&self.0
	}
	///
	///
	///
	pub fn len(&self) -> usize { self.0.len() }
	///
	///
	///
	pub fn zeros(&self) -> usize { 
		self.0.zeros()
	}
	///
	///
	///
	pub fn ones(&self) -> usize { 
		self.0.ones()
	}
	///
	///
	///
	pub fn show(&self) {
		print!("V[");
		for i in self.0.iter() {
            if i == true { print!("{:?}", 1); } else { print!("{:?}", 0); }
        }
    	println!("]");
	}
}

///
/// The `bitxor` operator for `Vector` of dimension `SIZE`.
// A	B	 A XOR B
// 0	0		0
// 0	1		1
// 1	0		1
// 1	1		0
///
impl BitXor for Vector {
    type Output = Self;

    fn bitxor(self, Vector(rhs): Self) -> Self {
        let Vector(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());
        let xor = lhs^rhs;
        println!("xor.len() = {:?}", xor.len());
        Vector(xor)
    }
}
///
/// The `bitand` operator for `Vector` of dimension `SIZE`.
// A	B	 A AND B
// 0	0		0
// 0	1		0
// 1	0		0
// 1	1		1
///
impl BitAnd for Vector {
    type Output = Self;

    fn bitand(self, Vector(rhs): Self) -> Self {
        let Vector(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());

        Vector(lhs&rhs)
    }
}
///
/// The `bitor` operator for `Vector` of dimension `SIZE`.
// A	B	  A OR B
// 0	0		0
// 0	1		1
// 1	0		1
// 1	1		1
///
impl BitOr for Vector {
    type Output = Self;

    fn bitor(self, Vector(rhs): Self) -> Self {
        let Vector(lhs) = self;
        assert_eq!(lhs.len(), rhs.len());

        Vector(lhs|rhs)
    }
}

#[cfg(test)]
mod tests {

	use std::mem;
    use super::Vector;
    use super::bits;
    use super::SDM_DIM;

    #[test]
    fn test_vector_size() {
        println!("size_of<Vector()> = {} in B.\n", mem::size_of::<Vector>());
    }

    #[test]
    fn test_vector_new() {
        let a: Vector = Vector::new();
        assert_eq!(a.len(), SDM_DIM*bits());
    } 

    #[test]
    fn test_vector_xor() {
        let a: Vector = Vector::new();
        let al = a.len();
        assert_eq!(al, SDM_DIM*bits());
        //a.show();
        let b: Vector = Vector::new();
        let bl = b.len();
        assert_eq!(al, bl);
        //b.show();
        let c = a^b;
        let cl = c.len();
        //c.show();
        assert_eq!(cl, al);
    } 
}