//!
//! [`BitTable`] is a bit-vector, a vector of single-bit values stored compactly in memory.
//! BitTable grows dynamically, like the standard `Vec<T>` type.  
//! It can hold up to about one word of bits inline without a separate heap allocation.  
//! `mem::size_of::<usize>() = 8` bytes on a 64 bits target.
//! If the number of bits exceeds this inline capacity, it will allocate a buffer on the heap.
//!
//! [`BitTable`]: struct.BitTable.html
//!
//! # Example
//!
//! ```
//! use bittable::BitTable;
//!
//! let mut bt = BitTable::new();
//! bt.push(true);
//! bt.push(false);
//!
//! assert_eq!(bt[0], true);
//! assert_eq!(bt[1], false);
//! ```
///
/// Import 'array_float_constants.rs'.
/// The file was generated with values defined in 'build.rs'.
///
//#[allow(missing_docs)]
//include!(concat!(env!("OUT_DIR"), "/constants.rs"));

use std::cmp::max;
use std::fmt;
use std::hash;
use std::iter::{DoubleEndedIterator, ExactSizeIterator, FromIterator};
use std::mem::{forget, replace, size_of};
use std::ops::{Index, Range};
use std::slice;

/// Creates a [`BitTable`] containing the arguments.
///
/// `btvec!` allows `BitTable`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// - Create a [`BitTable`] containing a given list of elements:
///
/// ```
/// #[macro_use]
/// use bittable::BitTable;
/// fn main() {
///     let v = btvec![true, false, true];
///     assert_eq!(v[0], true);
///     assert_eq!(v[1], false);
///     assert_eq!(v[2], true);
/// }
/// ```
///
/// - Create a [`BitTable`] from a given element and size:
///
/// ```
/// # #[macro_use]
/// use bittable::BitTable;
/// # fn main() {
///     let v = btvec![true; 3];
///     assert!(v.into_iter().eq(vec![true, true, true].into_iter()));
/// # }
/// ```
#[macro_export]
macro_rules! btvec {
    ($elem:expr; $n:expr) => ($crate::BitTable::from_elem($n, $elem));
    ($($x:expr),*) => ([$($x),*].iter().cloned().collect::<$crate::BitTable>());
    ($($x:expr,)*) => (btvec![$($x),*]);
}
///
/// A resizable bit vector, optimized for size and inline storage.
///
/// `BitTable` is exactly one word wide. 
/// Depending on the required capacity, this word either stores the bits on the stack,
/// or it stores a pointer to a separate buffer on the heap.
///
pub struct BitTable { data: usize }
///
/// Total number of bits per word.
///
/// The size of `usize` is how many bytes it takes to reference any location in memory. 
///
/// Note that `size_of::<usize>() = 8` bytes on a 64 bits target. 
/// The size is then `8 * 8 = 64 bits`.
///
#[inline(always)]
fn inline_bits() -> usize { size_of::<usize>() * 8 }
///
// For an inline vector, all bits except two can be used as storage capacity:
//
// bits: 64 - 2 
//
//      0                             ||62 63 
//     |   |  |  |  |  |  |  |  |  |  ||  |  | 
//                                ^   ||    ^
//                                |         |_____ 
//                                |                0: stack 
//                                |____            1: heap
//                                     length 
//
// - The rightmost bit is set to zero to signal an inline vector.
// - The position of the rightmost nonzero bit encodes the length.
/// The inline capacity is then `64 - 2 = 62 bits`.
#[inline(always)]
fn inline_capacity() -> usize { inline_bits() - 2 }
///
/// Left shift amount to access the `n`th bit
///
#[inline(always)]
fn inline_shift(n: usize) -> usize {
    debug_assert!(n <= inline_capacity());
    // The storage starts at the leftmost bit.
    inline_bits() - 1 - n
}
///
/// An inline vector with the `n`th bit set.
///
#[inline(always)]
fn inline_index(n: usize) -> usize { 1 << inline_shift(n) }
///
/// An inline vector with the leftmost `n` bits set.
///
#[inline(always)]
fn inline_ones(n: usize) -> usize {
    if n == 0 { 0 } else { !0 << (inline_bits() - n) }
}
///
/// If the rightmost bit of `data` is set, then the remaining bits of `data`
/// are a pointer to a heap allocation.
///
const HEAP_FLAG: usize = 1usize;
///
/// The allocation will contain a `Header` followed by a [Storage] buffer.
///
type Storage = usize;
///
/// The number of bits in one `Storage`.
///
#[inline(always)]
fn bits_per_storage() -> usize { size_of::<Storage>() * 8 }
/// Data stored at the start of the heap allocation.
///
/// `Header` must have the same alignment as `Storage`.
struct Header {
    /// The number of bits in this bit vector.
    len: Storage,
    /// The number of elements in the [usize] buffer that follows this header.
    buffer_len: Storage,
}
///
///
///
impl Header {
    ///
    /// Create a heap allocation with enough space for a header,
    /// plus a buffer of at least `cap` bits, each initialized to `val`.
    fn new(cap: usize, len: usize, val: bool) -> *mut Header {
        let alloc_len = header_len() + buffer_len(cap);
        let init = if val { !0 } else { 0 };

        let v: Vec<Storage> = vec![init; alloc_len];

        let buffer_len = v.capacity() - header_len();
        let header_ptr = v.as_ptr() as *mut Header;

        forget(v);

        unsafe {
            (*header_ptr).len = len;
            (*header_ptr).buffer_len = buffer_len;
        }
        header_ptr
    }
}
///
/// The number of `Storage` elements to allocate to hold a header.
/// 
#[inline(always)]
fn header_len() -> usize { size_of::<Header>() / size_of::<Storage>() }
///
///
/// The minimum number of `Storage` elements to hold at least `cap` bits.
#[inline(always)]
fn buffer_len(cap: usize) -> usize { (cap + bits_per_storage() - 1) / bits_per_storage() }
///
///
///
impl BitTable {
    ///
    /// Create an empty vector.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use bittable::BitTable;
    ///
    /// let bt = BitTable::new();
    /// ```
    ///
    #[inline]
    pub fn new() -> BitTable { BitTable { data: inline_index(0), } }
    ///
    /// Create a vector containing `len` bits, each set to `val`.
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5);
/// ```
    ///
    #[inline]
    pub fn from_elem(len: usize, val: bool) -> BitTable {
        if len <= inline_capacity() {
            return BitTable {
                data: if val {
                    inline_ones(len + 1)
                } else {
                    inline_index(len)
                },
            };
        }
        let header_ptr = Header::new(len, len, val);
        BitTable { data: (header_ptr as usize) | HEAP_FLAG, }
    }
    ///
    /// Create an empty vector with enough storage pre-allocated to store at least `cap` bits
    /// without resizing.
    #[inline]
    pub fn with_capacity(cap: usize) -> BitTable {
        // Use inline storage if possible.
        if cap <= inline_capacity() {
            return BitTable::new();
        }
        // Otherwise, allocate on the heap.
        let header_ptr = Header::new(cap, 0, false);
        BitTable {
            data: (header_ptr as usize) | HEAP_FLAG,
        }
    }
    ///
    /// The number of bits stored in this bit vector.
    ///
    #[inline]
    pub fn len(&self) -> usize {
        if self.is_inline() {
            // The rightmost nonzero bit is a sentinel.  All bits to the left of
            // the sentinel bit are the elements of the bit vector.
            inline_bits() - self.data.trailing_zeros() as usize - 1
        } else {
            self.header().len
        }
    }

    /// Returns `true` if this vector contains no bits.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The number of bits that can be stored in this bit vector without re-allocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        if self.is_inline() {
            inline_capacity()
        } else {
            self.header().buffer_len * bits_per_storage()
        }
    }

    /// Get the nth bit in this bit vector.
    #[inline]
    pub fn get(&self, n: usize) -> Option<bool> {
        if n < self.len() {
            Some(unsafe { self.get_unchecked(n) })
        } else {
            None
        }
    }

    /// Get the nth bit in this bit vector, without bounds checks.
    #[inline]
    pub unsafe fn get_unchecked(&self, n: usize) -> bool {
        if self.is_inline() {
            self.data & inline_index(n) != 0
        } else {
            let buffer = self.buffer();
            let i = n / bits_per_storage();
            let offset = n % bits_per_storage();
            *buffer.get_unchecked(i) & (1 << offset) != 0
        }
    }

    /// Set the nth bit in this bit vector to `val`.  Panics if the index is out of bounds.
    #[inline]
    pub fn set(&mut self, n: usize, val: bool) {
        assert!(n < self.len(), "Index {} out of bounds", n);
        unsafe {
            self.set_unchecked(n, val);
        }
    }

    /// Set the nth bit in this bit vector to `val`, without bounds checks.
    #[inline]
    pub unsafe fn set_unchecked(&mut self, n: usize, val: bool) {
        if self.is_inline() {
            if val {
                self.data |= inline_index(n);
            } else {
                self.data &= !inline_index(n);
            }
        } else {
            let buffer = self.buffer_mut();
            let i = n / bits_per_storage();
            let offset = n % bits_per_storage();
            if val {
                *buffer.get_unchecked_mut(i) |= 1 << offset;
            } else {
                *buffer.get_unchecked_mut(i) &= !(1 << offset);
            }
        }
    }

    /// Append a bit to the end of the vector.
    ///
    /// ```
    /// use BitTable::BitTable;
    /// let mut v = BitTable::new();
    /// v.push(true);
    ///
    /// assert_eq!(v.len(), 1);
    /// assert_eq!(v.get(0), Some(true));
    /// ```
    #[inline]
    pub fn push(&mut self, val: bool) {
        let idx = self.len();
        if idx == self.capacity() {
            self.reserve(1);
        }
        unsafe {
            self.set_len(idx + 1);
            self.set_unchecked(idx, val);
        }
    }

    /// Remove the last bit from the vector and return it, if there is one.
    ///
    /// # Examples
    ///
    /// ```
    /// use bittable::BitTable;
    ///
    /// let mut v = BitTable::new();
    /// v.push(false);
    ///
    /// assert_eq!(v.pop(), Some(false));
    /// assert_eq!(v.len(), 0);
    /// assert_eq!(v.pop(), None);
    /// ```
    /// # Arguments
    ///
    /// * `&mut self`
    #[inline]
    pub fn pop(&mut self) -> Option<bool> {
        let old_len = self.len();
        if old_len == 0 {
            return None;
        }
        unsafe {
            let val = self.get_unchecked(old_len - 1);
            self.set_len(old_len - 1);
            Some(val)
        }
    }

    /// Remove and return the bit at index `idx`, shifting all later bits toward the front.
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    pub fn remove(&mut self, idx: usize) -> bool {
        let len = self.len();
        let val = self[idx];

        if self.is_inline() {
            // Shift later bits, including the length bit, toward the front.
            let mask = !inline_ones(idx);
            let new_vals = (self.data & mask) << 1;
            self.data = (self.data & !mask) | (new_vals & mask);
        } else {
            let first = idx / bits_per_storage();
            let offset = idx % bits_per_storage();
            let count = buffer_len(len);
            {
                // Shift bits within the first storage block.
                let buf = self.buffer_mut();
                let mask = !0 << offset;
                let new_vals = (buf[first] & mask) >> 1;
                buf[first] = (buf[first] & !mask) | (new_vals & mask);
            }
            // Shift bits in subsequent storage blocks.
            for i in (first + 1)..count {
                // Move the first bit into the previous block.
                let bit_idx = i * bits_per_storage();
                unsafe {
                    let first_bit = self.get_unchecked(bit_idx);
                    self.set_unchecked(bit_idx - 1, first_bit);
                }
                // Shift the remaining bits.
                self.buffer_mut()[i] >>= 1;
            }
            // Decrement the length.
            unsafe {
                self.set_len(len - 1);
            }
        }
        val
    }

    /// Remove all elements from the vector, without deallocating its buffer.
    #[inline]
    pub fn clear(&mut self) {
        unsafe {
            self.set_len(0);
        }
    }

    /// Reserve capacity for at least `additional` more elements to be inserted.
    ///
    /// May reserve more space than requested, to avoid frequent reallocations.
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// Re-allocates only if `self.capacity() < self.len() + additional`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let old_cap = self.capacity();
        let new_cap = self.len()
        .checked_add(additional)
        .expect("capacity overflow");
        if new_cap <= old_cap {
            return;
        }
        // Ensure the new capacity is at least double, to guarantee exponential growth.
        let double_cap = old_cap.saturating_mul(2);
        self.reallocate(max(new_cap, double_cap));
    }

    /// Set the length of the vector. The length must not exceed the capacity.
    ///
    /// If this makes the vector longer, then the values of its new elements
    /// are not specified.
    #[inline]
    unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= self.capacity());
        if self.is_inline() {
            let sentinel = inline_index(len);
            let mask = !(sentinel - 1);
            self.data |= sentinel;
            self.data &= mask;
        } else {
            self.header_mut().len = len;
        }
    }

    /// Returns an iterator that yields the bits of the vector in order, as `bool` values.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {
            vec: self,
            range: 0..self.len(),
        }
    }
    ///
    /// Returns an immutable view of a range of bits from this vec.
    ///
    /// # Examples
    ///
    /// ```
    /// use bittable::BitTable;
    /// #[macro_use]
    /// let v = btvec![true, false, true];
    /// let r = v.range(1..3);
    /// assert_eq!(r[1], true);
    /// ```
    #[inline]
    pub fn range(&self, range: Range<usize>) -> VecRange {
        assert!(range.end <= self.len(), "range out of bounds");
        VecRange { vec: &self, range }
    }

    /// Returns true if all the bits in the vec are set to zero/false.
    #[inline]
    pub fn all_false(&self) -> bool {
        let mut len = self.len();
        if len == 0 {
            return true;
        }

        if self.is_inline() {
            let mask = inline_ones(len);
            self.data & mask == 0
        } else {
            for &storage in self.buffer() {
                if len >= bits_per_storage() {
                    if storage != 0 {
                        return false;
                    }
                    len -= bits_per_storage();
                } else {
                    let mask = (1 << len) - 1;
                    if storage & mask != 0 {
                        return false;
                    }
                    break;
                }
            }
            true
        }
    }

    /// Returns true if all the bits in the vec are set to one/true.
    #[inline]
    pub fn all_true(&self) -> bool {
        let mut len = self.len();
        if len == 0 {
            return true;
        }

        if self.is_inline() {
            let mask = inline_ones(len);
            self.data & mask == mask
        } else {
            for &storage in self.buffer() {
                if len >= bits_per_storage() {
                    if storage != !0 {
                        return false;
                    }
                    len -= bits_per_storage();
                } else {
                    let mask = (1 << len) - 1;
                    if storage & mask != mask {
                        return false;
                    }
                    break;
                }
            }
            true
        }
    }

    /// Resize the vector to have capacity for at least `cap` bits.
    ///
    /// `cap` must be at least as large as the length of the vector.
    fn reallocate(&mut self, cap: usize) {
        let old_cap = self.capacity();
        if cap <= old_cap {
            return;
        }
        assert!(self.len() <= cap);

        if self.is_heap() {
            let old_buffer_len = self.header().buffer_len;
            let new_buffer_len = buffer_len(cap);

            let old_alloc_len = header_len() + old_buffer_len;
            let new_alloc_len = header_len() + new_buffer_len;

            let old_ptr = self.header_raw() as *mut Storage;
            let mut v = unsafe { Vec::from_raw_parts(old_ptr, old_alloc_len, old_alloc_len) };
            v.resize(new_alloc_len, 0);
            v.shrink_to_fit();
            self.data = v.as_ptr() as usize | HEAP_FLAG;
            forget(v);

            self.header_mut().buffer_len = new_buffer_len;
        } else {
            let old_self = replace(self, BitTable::with_capacity(cap));
            unsafe {
                self.set_len(old_self.len());
                for i in 0..old_self.len() {
                    self.set_unchecked(i, old_self.get_unchecked(i));
                }
            }
        }
    }

    /// If the vector owns a heap allocation, returns a pointer to the start of the allocation.
    ///
    /// The layout of the data at this allocation is a private implementation detail.
    #[inline]
    pub fn heap_ptr(&self) -> Option<*const usize> {
        if self.is_heap() {
            Some((self.data & !HEAP_FLAG) as *const Storage)
        } else {
            None
        }
    }
    ///
    /// If the rightmost bit is set, then we treat it as inline storage.
    #[inline]
    fn is_inline(&self) -> bool { self.data & HEAP_FLAG == 0 }
    ///
    /// Otherwise, `data` is a pointer to a heap allocation.
    #[inline]
    fn is_heap(&self) -> bool { !self.is_inline() }
    ///
    /// Get the header of a heap-allocated vector.
    #[inline]
    fn header_raw(&self) -> *mut Header {
        assert!(self.is_heap());
        (self.data & !HEAP_FLAG) as *mut Header
    }

    #[inline]
    fn header_mut(&mut self) -> &mut Header {
        unsafe { &mut *self.header_raw() }
    }

    #[inline]
    fn header(&self) -> &Header {
        unsafe { &*self.header_raw() }
    }

    /// Get the buffer of a heap-allocated vector.
    #[inline]
    fn buffer_raw(&self) -> *mut [Storage] {
        unsafe {
            let header_ptr = self.header_raw();
            let buffer_len = (*header_ptr).buffer_len;
            let buffer_ptr = (header_ptr as *mut Storage)
            .offset((size_of::<Header>() / size_of::<Storage>()) as isize);
            slice::from_raw_parts_mut(buffer_ptr, buffer_len)
        }
    }

    #[inline]
    fn buffer_mut(&mut self) -> &mut [Storage] {
        unsafe { &mut *self.buffer_raw() }
    }

    #[inline]
    fn buffer(&self) -> &[Storage] {
        unsafe { &*self.buffer_raw() }
    }
}
// Trait implementations:
///
///
///
impl fmt::Debug for BitTable {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_list()
        .entries(self.iter().map(|b| b as u8))
        .finish()
    }
}
///
///
///
impl Default for BitTable {
    fn default() -> Self {
        Self::new()
    }
}
///
///
///
impl PartialEq for BitTable {
    fn eq(&self, other: &Self) -> bool {
        // Compare by inline representation
        if self.is_inline() && other.is_inline() {
            return self.data == other.data;
        }

        let len = self.len();
        if len != other.len() {
            return false;
        }

        // Compare by heap representation
        if self.is_heap() && other.is_heap() {
            let buf0 = self.buffer();
            let buf1 = other.buffer();

            let full_blocks = len / bits_per_storage();
            let remainder = len % bits_per_storage();

            if buf0[..full_blocks] != buf1[..full_blocks] {
                return false;
            }

            if remainder != 0 {
                let mask = (1 << remainder) - 1;
                if buf0[full_blocks] & mask != buf1[full_blocks] & mask {
                    return false;
                }
            }
            return true;
        }

        // Representations differ; fall back to bit-by-bit comparison
        Iterator::eq(self.iter(), other.iter())
    }
}

impl Eq for BitTable {}

impl Drop for BitTable {
    fn drop(&mut self) {
        if self.is_heap() {
            unsafe {
                let header_ptr = self.header_raw();
                let alloc_ptr = header_ptr as *mut Storage;
                let alloc_len = header_len() + (*header_ptr).buffer_len;
                Vec::from_raw_parts(alloc_ptr, alloc_len, alloc_len);
            }
        }
    }
}

impl Clone for BitTable {
    fn clone(&self) -> Self {
        if self.is_inline() {
            return BitTable { data: self.data };
        }

        let buffer_len = self.header().buffer_len;
        let alloc_len = header_len() + buffer_len;
        let ptr = self.header_raw() as *mut Storage;
        let raw_allocation = unsafe { slice::from_raw_parts(ptr, alloc_len) };

        let v = raw_allocation.to_vec();
        let header_ptr = v.as_ptr() as *mut Header;
        forget(v);
        BitTable {
            data: (header_ptr as usize) | HEAP_FLAG,
        }
    }
}

impl Index<usize> for BitTable {
    type Output = bool;

    #[inline(always)]
    fn index(&self, i: usize) -> &bool {
        assert!(i < self.len(), "index out of range");
        if self.get(i).unwrap() {
            &true
        } else {
            &false
        }
    }
}

impl hash::Hash for BitTable {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for b in self.iter() {
            b.hash(state);
        }
    }
}

impl Extend<bool> for BitTable {
    #[inline]
    fn extend<I: IntoIterator<Item = bool>>(&mut self, iter: I) {
        let iter = iter.into_iter();

        let (min, _) = iter.size_hint();
        assert!(min <= usize::max_value(), "capacity overflow");
        self.reserve(min);

        for element in iter {
            self.push(element)
        }
    }
}

impl FromIterator<bool> for BitTable {
    #[inline]
    fn from_iter<I: IntoIterator<Item = bool>>(iter: I) -> Self {
        let mut v = BitTable::new();
        v.extend(iter);
        v
    }
}

impl IntoIterator for BitTable {
    type Item = bool;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> IntoIter {
        IntoIter {
            range: 0..self.len(),
            vec: self,
        }
    }
}

impl<'a> IntoIterator for &'a BitTable {
    type Item = bool;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

/// An iterator that owns a `BitTable` and yields its bits as `bool` values.
///
/// Returned from [`BitTable::into_iter`][1].
///
/// [1]: struct.BitTable.html#method.into_iter
#[derive(Debug)]
pub struct IntoIter {
    vec: BitTable,
    range: Range<usize>,
}
///
///
///
impl Iterator for IntoIter {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.range
        .next()
        .map(|i| unsafe { self.vec.get_unchecked(i) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl DoubleEndedIterator for IntoIter {
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        self.range
        .next_back()
        .map(|i| unsafe { self.vec.get_unchecked(i) })
    }
}

impl ExactSizeIterator for IntoIter {}

/// An iterator that borrows a BitTable and yields its bits as `bool` values.
///
/// Returned from [`BitTable::iter`][1].
///
/// [1]: struct.BitTable.html#method.iter
#[derive(Debug)]
pub struct Iter<'a> {
    vec: &'a BitTable,
    range: Range<usize>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        self.range
        .next()
        .map(|i| unsafe { self.vec.get_unchecked(i) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<'a> DoubleEndedIterator for Iter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<bool> {
        self.range
        .next_back()
        .map(|i| unsafe { self.vec.get_unchecked(i) })
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}

/// An immutable view of a range of bits from a borrowed BitTable.
///
/// Returned from [`BitTable::range`][1].
///
/// [1]: struct.BitTable.html#method.range
#[derive(Debug, Clone)]
pub struct VecRange<'a> {
    vec: &'a BitTable,
    range: Range<usize>,
}
///
///
///

impl<'a> VecRange<'a> {
    ///
    ///
    ///
    #[inline]
    pub fn iter(&self) -> Iter<'a> {
        Iter {
            vec: self.vec,
            range: self.range.clone(),
        }
    }
}

impl<'a> Index<usize> for VecRange<'a> {
    type Output = bool;

    #[inline]
    fn index(&self, i: usize) -> &bool {
        let vec_i = i + self.range.start;
        assert!(vec_i < self.range.end, "index out of range");
        &self.vec[vec_i]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn test_inline_capacity() {
        assert_eq!(inline_capacity(), 30);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn test_inline_capacity() {
        assert_eq!(inline_capacity(), 62);
    }

}