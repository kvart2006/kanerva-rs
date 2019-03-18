//
// Import 'array_float_constants.rs'.
// The file was generated with values defined in 'build.rs'.
//
#[allow(missing_docs)]
include!(concat!(env!("OUT_DIR"), "/dimensions.rs"));
///
use bittable::BitTable;
///
///
/// Space of addresses.
///
/// Dimensions read in configuration file: DIM and HARDLOC.
/// 1: DIM = The dimension of addresses.
/// 2: HARDLOC = Number of hard-locations.
///
/// This approach allocates a continuous chunk of memory for all addresses.
/// The `addresses` allows the use of array notation: addresses[0], addresses[1], ...
/// Let `a` be `AddressSpace`: 
///
/// `        		a[0]   a[1]   a[2]   a[3]   a[4]`
/// `				 |      |      |      |      |`
/// `		 		 v      v      v      v      v`
/// `AddressSpace = xxxxxx|xxxxxx|xxxxxx|xxxxxx|xxxxxx`
///
/// row: SIZE*8*mem::size_of::<usize>()
/// col: HARDLOC*8*mem::size_of::<usize>()
///
///
///
#[derive(Clone, Debug)]
pub struct AddressSpace (Vec<BitTable>);
///
///
///
impl AddressSpace {
	///
    /// # Examples
    ///
    /// ```rust, ignore
    /// use address_space::AddressSpace;
    /// 
    /// let a = AddressSpace::new();
    /// ```
	pub fn new() -> Self {
        let mut vec: Vec<BitTable> = Vec::with_capacity(ADD_DIM);
        for _ in 0..ADD_DIM {
            vec.push(BitTable::new_rand());
        }

        let mut counter: usize = 0;
        let mut d: usize;

        for i in 0..ADD_DIM {
            println!("vec[{}].ones()={}", i, vec[i].ones()); 
        }
        println!("vec.len()={}", vec.len()); 

        let mut b=BitTable::new_rand();

        println!(); 
        for i in 0..ADD_DIM { 
            d = b.distance(&vec[i]);
            println!("i={}:d={}", i, d); 
            if d < 128 { counter += 1; }
        }
        println!(" << distance = {} =================================", counter);
        println!(); 
        AddressSpace(vec)
	}
	///
	///
	///
	pub fn len(&self) -> usize { self.0.len() }
    ///
    ///
    /// 
    pub fn get(& self, i: usize) -> &BitTable { &self.0[i] }   
	///
	///
	///	
	pub fn set(&mut self, i: usize, bt: &BitTable) {
        self.0[i] = bt.clone();
	}
    ///
    /// 
    ///
    pub fn show(&mut self) {
        print!("AS[");
        for i in 0..ADD_DIM { self.0[i].show(); println!(""); }
        println!("]");
    } 
    ///
    /// 
    ///
    pub fn write(&mut self) {
        print!("AS[");
        for i in 0..ADD_DIM { self.0[i].show(); println!(""); }
        println!("]");
    } 
    ///
    /// 
    ///
    pub fn read(&mut self) {
        print!("AS[");
        for i in 0..ADD_DIM { self.0[i].show(); println!(""); }
        println!("]");
    } 
    ///
    /// 
    ///
    pub fn radius_counter(&self, bt: &mut BitTable, radius: usize) -> usize {
        let mut counter: usize = 0;
        for it in self.0.iter() {
            let d = bt.distance(it);
            println!("d[{}]", d);
            if d < radius { counter+=1; }
        }
        counter
    }
    ///
    /// 
    ///
    pub fn radius_fit(&self, bt: &BitTable, radius: usize) -> Vec<usize> {
        
        let mut selected: Vec<usize> = vec![0;ADD_DIM];
        for i in 0..ADD_DIM {  
            let x: BitTable = self.0[i] ^ *bt;
            if x.zeros() <= radius { selected[i]+=1; }
        }
        selected
    }
    ///
    ///
    ///
    pub fn xor_in_place<'a>(&self, left: &'a [bool], right: &'a [bool]) -> Vec<bool> {
        assert_eq!(left.len(), right.len());

        let it = right.iter().cycle();
        left.iter().zip(it).map(|(&a, b)| a ^ b ).collect()
    }

}

#[cfg(test)]
mod tests {

    use ADD_DIM;
    use DIM;
    use super::AddressSpace;
    use super::BitTable;

    #[test]
    fn test_address_space_new() {
        let a: AddressSpace = AddressSpace::new();
        println!("address_space_new() = {}", a.len());
        assert_eq!(a.len(), ADD_DIM);
    } 

    #[test]
    fn test_address_space_radius_counter() {
        let a: AddressSpace = AddressSpace::new();
        let mut bt: BitTable = BitTable::new_rand();
        let radius: usize = 64;
        let c = a.radius_counter(&mut bt, radius);
        println!("radius({}) >= {}", radius, c);
    }

   #[test]
    fn test_address_space_radius_fit() {
        let a: AddressSpace = AddressSpace::new();
        let bt: BitTable = BitTable::new_rand();
        let radius: usize = 64;
        let c: Vec<usize> = a.radius_fit(&bt, radius);
        println!("\nradius >= {:?}.len()", c.len());
    }
/*
    #[test]
    fn test_address_space_show() {
        let mut a: AddressSpace = AddressSpace::new();
        a.show()
    }
*/
/*
    #[test]
    fn test_address_space_scan() {
        let mut a: AddressSpace = AddressSpace::new();
        a.show();
        let b: Vector = Vector::new();
        b.show();
        let radius: u32 = 1;
        let selected = a.scan(&b, radius);
        print!("S[");
        for i in selected.iter() {
        	print!("{:?}", i);
        }
    	println!("]");
        //let i: usize = 2;
        //let ba: BoolArray = *a.get(i);
        //for x in 0..ba.len() { if ba.get(x) == false { print!("0") } else { print!("1") } }; 
    } 
*/
}