#[test]
fn readme_1() {
	use funty::num::Unsigned;
	fn invert_middle_bits<T: Unsigned>(num: T) -> T {
		let mask = (T::MAX << 2) & (T::MAX >> 2);
		num ^ mask
	}
	assert_eq!(invert_middle_bits(0xAAu8), 0b1001_0110u8);
}

#[test]
fn readme_2() {
	use funty::ptr::*;
	let data = 0u32;
	let raw: *const u32 = &data;
	let ptr = Pointer::from_const(raw);
	assert!(ptr.make_mut().is_err());
	assert!(!raw.cast_mut().is_null());
	assert_eq!(unsafe { ptr.read() }, 0);
}
