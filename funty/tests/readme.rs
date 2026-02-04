#[test]
fn readme_1() {
	use funty::num::Unsigned;
	fn invert_middle_bits<T: Unsigned>(num: T) -> T {
		let mask = (T::MAX << 2) & (T::MAX >> 2);
		num ^ mask
	}
	assert_eq!(invert_middle_bits(0xAAu8), 0b1001_0110u8);
}
