use funty::Unsigned;

#[test]
fn readme() {
	fn invert_some_bits<T: Unsigned>(num: T) -> T {
		let mask = (!T::ZERO).wrapping_shl(2).wrapping_shr(4).wrapping_shl(2);
		num ^ mask
	}
	assert_eq!(invert_some_bits(!0u8), 0b1100_0011u8);
}
