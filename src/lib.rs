//! # Library Kanerva
//!
//! `Kanerva` is a compilation time dimensioned array-based implementation.
//! 
//! Two parameters are needed:
//!   1: SDM dimension.
//!   2: Number of hard-locations.
//! 
//! Example: 1: 1,000, 2: 1,000,000:
//! generates an address space with 1,000,000 random 1,000-bit arrays.
//!
//! Addresses are stored as 'addresses', and can be accessed as
//! 'addresses[0]', 'addresses[1]',..
//! 
//! On OS X the stack size is limited to:
//! `ulimit -Hs = 65532 KB` hard limit
//! `ulimit -Ss =  8192 KB` soft limit
//!
//! let DIM: usize = 256;
//! let HARDLOC: usize = 10000;
//! 
//! Size of the setup = 5120 KB - (5120000 B).
//! MAX: i16 = 32767 with size 2 bytes

#![crate_type = "lib"]
#![feature(test)]
#![feature(core_intrinsics)]
#![feature(ptr_offset_from)]
#![feature(alloc, raw_vec_internals)]
#![feature(exact_size_is_empty)]
#![feature(const_fn)]

#![deny(
  missing_docs,
  missing_debug_implementations,
  missing_copy_implementations,
  trivial_casts,
  trivial_numeric_casts,
  unused_import_braces,
  unused_qualifications
  )]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(missing_docs)]
include!(concat!(env!("OUT_DIR"), "/dimensions.rs"));

extern crate rand;
extern crate libc;
extern crate bittable;
///
///
///
pub mod address_space;
///
///
///
pub mod counter;
///
use rand::{Rng, thread_rng};
///
use address_space::AddressSpace;
///
use counter::Counter;
///
use bittable::BitTable;
/// 
/// Structure SDM.
///
#[derive(Clone, Debug)]
pub struct SDM {
    d: usize,
    hl: usize,
    a: AddressSpace,
    c: Counter,
}
///
///
///
impl SDM {
    ///
    ///
    ///
    pub fn new() -> Self {
        SDM { d: DIM, hl: ADD_DIM, a: AddressSpace::new(), c: Counter::new() }
    }
    ///
    ///
    ///
    pub fn scan(&self, b: &BitTable, radius: usize) -> Vec<usize> {
        AddressSpace::radius_fit(&self.a, b, radius)
    }
    ///
    ///
    ///
    pub fn write(&mut self, b: &BitTable, radius: usize) {
        let selected: Vec<usize> = AddressSpace::radius_fit(&self.a, b, radius);
        println!("WRITE selected len = {}, selected = {:?}\n", selected.len(), selected);
        for i in 0..ADD_DIM { 
            // Get the address inside radius.
            let bt = self.a.get(i);
            // Update counters at the address.
            self.c.input(i, bt); 
            println!("WRITE i = {}", i);
        }
    }
    ///
    ///
    ///
    pub fn read(&mut self, b: &BitTable, radius: usize) -> BitTable {
        let selected: Vec<usize> = AddressSpace::radius_fit(&self.a, b, radius);
        println!("READ selected len = {}, selected = {:?}\n", selected.len(), selected);
        let mut sum: Vec<i16> = vec![0;b.len()];
        let mut counters: Vec<i16>;
        let rng = &mut thread_rng();
        let mut bt: BitTable = BitTable::new();
        // For all selected addresses
        for i in 0..ADD_DIM { 
            // Counters for selected address `i`.
            counters = self.c.get_counters(i);
            // Sum the contents of the `i`th buckets
            for count in 0..counters.len() { 
                sum[count] += counters[count];
            }
        }
        println!("\n===================================\n");
        println!("\nSum[{:?}]", sum);
        println!("]\n===================================\n");
        // Threshold the sum to either 1 or 0 based on whether the sum is positive or negative
        for i in 0..sum.len() { 
            if sum[i] > 0 { sum[i] = 1; } 
            else if sum[i] < 0 { sum[i] = 0; } 
            else if sum[i] == 0 { 
                let v: bool=rng.gen(); 
                if v { sum[i] = 1;} else { sum[i] = 0; } 
            } 
        }
        println!("\n===================================\n");
        println!("\nThreshold[{:?}]", sum);
        println!("]\n===================================\n");
        for i in 0..sum.len() { if sum[i]>0 { bt.set(i) }}
        bt
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sdm_new() {
        let s: SDM = SDM::new();  
        assert_eq!(s.d, DIM);
    }

    #[test]
    fn test_sdm_scan() {
        let bt = BitTable::new_rand();
        let radius: usize = 128;
        let s: SDM = SDM::new();  
        let v = s.scan(&bt, radius);
        assert_eq!(s.d, DIM);
    }

    #[test]
    fn test_sdm_write() {
        let bt = BitTable::new_rand();
        let radius: usize = 128;
        let mut s: SDM = SDM::new();  
        let _ = s.write(&bt, radius);
        assert_eq!(s.d, DIM);
    }

    #[test]
    fn test_sdm_read() {
        let mut bt = BitTable::new_rand();
        let radius: usize = 36;
        let mut s: SDM = SDM::new();  
        s.write(&bt, radius);
        let v: BitTable = s.read(&bt, radius);
        //println!("SDM original:{:?}", bt);
        bt.show();
        v.show();

        let x=bt.distance(&v);
        println!("READ distance = {}", x);
    }


}
/*
///
///
///
impl SDM {
    ///
    ///
    ///
    pub fn new() -> Self {
        SDM { d: SIZE, hl: HARDLOC, a: AddressSpace::new(), c: Content::new() }
    }
    ///
    ///
    ///
    pub fn scan(a: &mut AddressSpace, b: &Vector, radius: u32) -> Vec<u32> {
        let mut selected: Vec<u32> = Vec::with_capacity(HARDLOC);
        //Vec<bool>
        let mut count: usize = Default::default();
        //let mut v: Vec<_> = self.0.chunks_mut(DIM).collect();
        //let bi = Vector::as_slice(b);
        // for i in 0..HARDLOC {
        //    let mut vi = AddressSpace::get(a, i);
        //    AddressSpace::xor_in_place(a, vi.as_slice(), bi);
        //}
        selected.truncate(count);
        selected
    }

}

#[cfg(test)]
mod tests {

    use std::mem;
    use super::DIM;
    use super::HARDLOC;
    use super::SDM;
    #[test]
    fn test_sdm_size() {
        println!("size_of<SDM({},{})> = {} in B.\n", DIM, HARDLOC, mem::size_of::<SDM>());
        println!("size_of<usize> = {} in B.\n", mem::size_of::<usize>());
        assert_eq!(8, mem::size_of::<usize>());
    }
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bittable::BitTable;
    ///
    /// let bt = BitTable::new();
    /// ```
    use super::BitTable;
    #[test]
    fn test_bittable_new() {
        let bt: BitTable = BitTable::new();
        assert_eq!(128, bt.len());
    }

    #[test]
    fn test_sdm_new() {
        let sdm: SDM = SDM::new();

    }
    */
/*
    use boolarray::BoolArray;
    #[test]
    fn test_boolarray_new() {
        let ba: BoolArray = BoolArray::new();
        //ba.show(None);
        println!("zeros = {}", ba.count_zeros());
        println!("ones  = {}", ba.count_ones());
    } 

    #[test]
    fn test_boolarray_new_from_vec() {
        if DIM == 5 {
            let ba: BoolArray = BoolArray::new_from_vec(vec![true, true, false, false, true].as_slice());
            //ba.show("new_from_vec");
            println!("zeros = {}", ba.count_zeros());
            println!("ones  = {}", ba.count_ones());
        } else { assert_eq!(DIM, DIM); }
    }  

    /// For SDM=4 only.
    #[test]
    fn test_boolarray_xor() {
        if DIM == 5 {
            let bv1 = BoolArray::new_from_vec(vec![true, true, false, false, true].as_slice());
            let bv2 = BoolArray::new_from_vec(vec![true, false, true, false, false].as_slice());
            let expected = BoolArray::new_from_vec(vec![false, true, true, false, true].as_slice());
            assert_eq!(bv1 ^ bv2, expected); 
        } else { assert_eq!(DIM, DIM); }

}

} */
/*
    use super::format_radix;
    #[test]
    fn test_format_radix() {
        println!("format_radix = {}", format_radix(1234, 10));
    }

    ///
    ///
    ///
    use super::BitArray;
    #[test]
    fn test_BitArray_new() {
        let bs : BitArray = BitArray::new();

        for i in 0..bs.len() { assert_eq!( format!("{:b}", bs.0[i]), "0" ) }
    } 
    ///
    ///
    ///
    #[test]
    fn test_BitArray_new_one() {
        let bs : BitArray = BitArray::new_one();
        for i in 0..bs.len() { assert_eq!( format!("{:b}", bs.0[i]), "1" ) }
    }   
    ///
    ///
    ///
    #[test]
    fn test_BitArray_new_selected() {
        let mut bs = BitArray::new();
        let i: usize = 0;
        bs.set(i);
        assert_eq!( bs.0[i], 1u8 );
    } 
    ///
    ///
    ///
    #[test]
    fn test_BitArray_new_rand() {
        let mut bs = BitArray::new_rand();
        let i: usize = 0;
        bs.set(i);
        assert_eq!( bs.0[i], 1u8 );
    } 
    ///
    ///
    ///
    #[test]
    fn test_BitArray_xor() {
        let bs1 = BitArray::new_rand();
        let bs2 = BitArray::new_rand();
        let bs = bs1^bs2;
        bs1.show();
        bs2.show();
        bs.show();
    } 
}



    use super::{BitArray_t, bs_alloc, bs_init_ones, bs_get_bit, bs_copy, bs_distance_naive, bs_init_random};
    use std::mem;
    ///
    /// u64::max_value() = 18446744073709551615
    /// 
    use random_init;
    #[test]
    fn test_random_init() {
        let bs : [u8; mem::size_of::<u64>()*SDM] = random_init();

        println!("  bs.len() = {} bytes", bs.len()); //
        print!("  bs = [");
        for i in bs.iter() { print!("{:b}", i); }
        print!("]");
        println!("");
    }
    ///
    /// u64::max_value() = 18446744073709551615
    /// 
    #[test]
    fn test_u64() {
        assert_eq!(u64::max_value(), 18446744073709551615);
    }
    ///
    /// u128::max_value() = 340282366920938463463374607431768211455
    /// 
    #[test]
    fn test_u128() {
        assert_eq!(u128::max_value(), 340282366920938463463374607431768211455);
    }

    use super::byteorder::*;
    ///
    /// u64::max_value() = 18446744073709551615
    /// Allows to represent [u8;64] bits
    /// 
    #[test]
    fn test_binary_u64() {
        let i: u64 = u64::max_value()-1;
        let mut bs = [0u8; mem::size_of::<u64>()]; // 1x8 bytes = 1x8x8 = 64 bits
        bs.as_mut().write_u64::<NativeEndian>(i).expect("Unable to write");
        println!("mem::size_of::<u64>() = {} bytes", mem::size_of::<u64>());
        println!("mem::size_of::<u8>() = {} bytes", mem::size_of::<u8>());
        println!("bs.len() = {} bytes", bs.len());
        print!("bs = [");
        for i in &bs { print!("{:b}", i); }
        print!("]");
        println!("");

        assert_eq!(i.count_ones(), 63);
        assert_eq!(i.count_zeros(), 1);
    }
    ///
    /// u64::max_value() = 18446744073709551615
    /// Allows to represent [u8;64*SDM9[p7
    /// 
    use rand::prelude::*;

    #[test]
    fn test_binary_u64_2() {
        let i: u64 = u64::max_value()-1;
        const bs_len: usize = SDM;
        let remaining_bits: usize = 0;
        let bits = (mem::size_of::<u64>() * 8) * bs_len - remaining_bits;
        println!("bits(u64) = {} bits", bits);
        let mut cz: u32 = 0;
        let mut co: u32 = 0;
        let mut count: u32 = 0;

        let mut bs = [0u8; mem::size_of::<u64>()*SDM]; // 8xSDM bytes = 8x8xSDM = 64xSDM bits
        print!("**bs = [");
        for i in bs.iter() { print!("{:b}", i); }
        print!("]");
        println!("");
        for i in 0..SDM { 
            println!("-----------------------------------------------------------{} ", i);
            let x = random::<u64>(); 
            let bx = format!("{:b}", x); 
            //let bbx = x.to_be().to_bytes();
            println!("x.count_ones()  = {}", x.count_ones());
            println!("x.count_zeros() = {}", x.count_zeros());
            co += x.count_ones();
            cz += x.count_zeros();
            //println!("bx(u64) = {}", bx);
            let min = i*mem::size_of::<u64>();
            let max = (i+1)*mem::size_of::<u64>();
            println!("min = {}, max = {}, x = {}, bx = {}", min, max, x, bx);

            //println!("min = {}, max = {}, x = {}, bx = {}", min, max, x, bx);
            if let Some(sub) = bs.get_mut(min .. max) { 
                sub.as_mut().write_u64::<NativeEndian>(x).expect("Unable to write") 
            };
            print!("****bs = [");
            for i in bs.iter() { print!("{:b}, ", i); }
            //for i in bs.iter() { print!("{:b}", i); }
            print!("]");
            println!("");
            //for i in 0..mem::size_of::<u64>() { bs =  *bx.as_bytes(); }
        }

        for i in 0..mem::size_of::<u64>()*SDM { println!("binary : {:b}", bs[i]); count += bs[i].count_zeros(); }

        println!("count_ones = {}, count_zeros = {}, count0 = {}", co, cz, count);
        
        //let mut rng = thread_rng();
        //for i in 0..mem::size_of::<u64>()*SDM { bs[i] =  rng.gen::<u8>(); }
        
        println!("bs.len() = {} bits", 8*bs.len()); //
        print!("*bs = [");
        for i in bs.iter() { print!("{:b}", i); }
        print!("]");
        println!("");

        assert_eq!(i.count_ones(), 63);
        assert_eq!(i.count_zeros(), 1);
    }
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 16];
    /// LittleEndian::write_u128(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u128(&buf));
    /// ```
    ///
    /// u128::max_value() = 340282366920938463463374607431768211455
    /// Allows to represent [u8;128] bits
    /// 
    #[test]
    fn test_binary_u128() {
        let i: u128 = u128::max_value()-1;
        let mut bs = [0u8; mem::size_of::<u128>()]; // 1x16 bytes = 1x16x8 = 128 bits
        bs.as_mut().write_u128::<NativeEndian>(i).expect("Unable to write");
        println!("mem::size_of::<u128>() = {} bytes", mem::size_of::<u128>());
        print!("bs = [");
        for i in &bs { print!("{:b}", i); }
        print!("]");
        println!("");

        assert_eq!(i.count_ones(), 127);
        assert_eq!(i.count_zeros(), 1);
    }
    ///
    /// bs_len=1, remaining_bits=0
    /// bs_len=1, bits=64
    /// 
    #[test]
    fn test_array_BitArray_t1() {
        let bs: *mut BitArray_t;
        let bs_len: ::std::os::raw::c_uint = 1;
        let remaining_bits: ::std::os::raw::c_uint = 0;
        println!("Testing mem::size_of::<u64>() [{}]...\n", mem::size_of::<u64>());
        let bits = (mem::size_of::<u64>() * 8) * bs_len as usize - remaining_bits as usize;
        println!("Testing bs_* functions [bs_len={}, bits={}]...\n", bs_len, bits);
        // Call functions.
        unsafe { 
            bs = bs_alloc(bs_len); 
            bs_init_random(bs, bs_len, remaining_bits);
        }
        print!("bs = [");
        unsafe { for i in 0..bits { print!("{:b}", bs_get_bit(bs, i as u32)); } }
        print!("]");
        println!("");
    }
    ///
    /// bs_len=2, remaining_bits=0
    /// bs_len=2, bits=128
    /// 
    #[test]
    fn test_array_BitArray_t2() {
        let bs: *mut BitArray_t;
        let bs_len: ::std::os::raw::c_uint = 2;
        let remaining_bits: ::std::os::raw::c_uint = 0;
        println!("Testing mem::size_of::<u64>() [{}]...\n", mem::size_of::<u64>());
        let bits = mem::size_of::<u64>() * 8 * bs_len as usize - remaining_bits as usize;
        println!("Testing bs_* functions [bs_len={}, bits={}]...\n", bs_len, bits);
        // Call functions.
        unsafe { 
            bs = bs_alloc(bs_len); 
            bs_init_ones(bs, bs_len, remaining_bits);
            for i in 0..bits { assert_eq!(bs_get_bit(bs, i as u32), 1) }
        }
        println!("");
    }
    ///
    /// bs_len=1, remaining_bits=30
    /// bs_len=1, bits=34
    /// 
    #[test]
    fn test_array_BitArray_t3() {
        let bs: *mut BitArray_t;
        let bs_len: ::std::os::raw::c_uint = 1;
        let remaining_bits: ::std::os::raw::c_uint = 30;
        println!("Testing mem::size_of::<u64>() [{}]...\n", mem::size_of::<u64>());
        let bits = mem::size_of::<u64>() * 8 * bs_len as usize - remaining_bits as usize;
        println!("Testing bs_* functions [bs_len={}, bits={}]...\n", bs_len, bits);
        // Call functions.
        unsafe { 
            bs = bs_alloc(bs_len); 
            bs_init_ones(bs, bs_len, remaining_bits);
            for i in 0..bits { assert_eq!(bs_get_bit(bs, i as u32), 1) }

            for i in 0..bits as u32 { print!("[{}]={}, ", i, bs_get_bit(bs, i)) }
        }
        println!("");
    }
    ///
    /// bs_len=26, remaining_bits=20
    /// bs_len=26, bits=1644
    /// 
    #[test]
    fn test_array_BitArray_t4() {
        let bs1: *mut BitArray_t;
        let bs2: *mut BitArray_t;
        let bs_len: ::std::os::raw::c_uint = 26;
        let remaining_bits: ::std::os::raw::c_uint = 20;
        println!("Testing mem::size_of::<u64>() [{}]...\n", mem::size_of::<u64>());
        let bits = mem::size_of::<u64>() * 8 * bs_len as usize - remaining_bits as usize;
        println!("Testing bs_* functions [bs_len={}, bits={}]...\n", bs_len, bits);
        // Call functions.
        unsafe { 
            bs1 = bs_alloc(bs_len); 
            bs_init_ones(bs1, bs_len, remaining_bits);
            for i in 0..bits { assert_eq!(bs_get_bit(bs1, i as u32), 1) }

            bs2 = bs_alloc(bs_len);
            bs_copy(bs2, bs1, bs_len);
            assert!(bs_distance_naive(bs1, bs2, bs_len) == 0);
        }
        println!("");
    }
    ///
    /// bs_len=1, remaining_bits=33
    /// bs_len=1, bits=31
    /// 
    #[test]
    fn test_array_BitArray_t5() {
        let bs: *mut BitArray_t;
        let bs_len: ::std::os::raw::c_uint = 1;
        let remaining_bits: ::std::os::raw::c_uint = 33;
        println!("Testing mem::size_of::<u64>() [{}]...\n", mem::size_of::<u64>());
        let bits = mem::size_of::<u64>() * 8 * bs_len as usize - remaining_bits as usize;
        println!("Testing bs_* functions [bs_len={}, bits={}]...\n", bs_len, bits);
        // Call functions.
        unsafe { 
            bs = bs_alloc(bs_len); 
            bs_init_random(bs, bs_len, remaining_bits);
            for i in 0..bits as u32 { print!("[{}]={}, ", i, bs_get_bit(bs, i)) }
        }
        println!("");
    }
    /// 
    ///
    ///
    use array::BoolArray;
    ///
    ///
    ///
    use array::SDM;
    #[test]
    fn test_array_mut() {

    }
    ///
    ///
    ///
    #[test]
    fn test_array_xor2() {
        let barray1: BoolArray = BoolArray::new();
        let barray2: BoolArray = BoolArray::new();
        let xorarray: BoolArray;

        //barray1.show();
        //barray2.show();

        xorarray = barray1 ^ barray2;
        //xorarray.show();
    }
        /*
    ///
    ///
    ///
    #[test]
    fn test_array_and() {
        let bv1 = BoolArray::new_from(&[true;1000]);
        let bv2 = BoolArray::new_from(&[true;1000]);
        let expected = BoolArray::new_from(&[true;1000]);
        assert_eq!(bv1 & bv2, expected);
    }
    ///
    ///
    ///
    #[test]
    fn test_array_and2() {
        let barray1: BoolArray = BoolArray::new();
        let barray2: BoolArray = BoolArray::new();
        let xorarray: BoolArray;

        //barray1.show();
        //barray2.show();

        xorarray = barray1 & barray2;
        //xorarray.show();
    }
    ///
    ///
    ///
    #[test]
    fn test_array_or() {
        let bv1 = BoolArray::new_from(&[true;1000]);
        let bv2 = BoolArray::new_from(&[true;1000]);
        let expected = BoolArray::new_from(&[true;1000]);
        assert_eq!(bv1 | bv2, expected);
    }
    */
}


*/