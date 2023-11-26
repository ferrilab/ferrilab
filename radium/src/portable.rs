//! Implements `Radium` on the `portable-atomic` types.

#![cfg(feature = "portable-atomic")]

pub use portable_atomic::*;

macro_rules! portable {
	($($width:literal : $bit:ident $num:ident => {
		$($(@<$t:ident>)? $base:ty => $atom:ident;)+
	})+) => { $( $(
		#[cfg(any(
			target_has_atomic = $width,
			feature = "portable-atomic-fallback",
		))]
		unsafe impl$(<$t>)? crate::Radium for $atom$(<$t>)? {
			type Item = $base;

			#[inline]
			fn new(value: $base) -> Self {
				<$atom$(<$t>)?>::new(value)
			}

			#[inline]
			fn fence(order: Ordering) {
				portable_atomic::fence(order);
			}

			#[inline]
			fn get_mut(&mut self) -> &mut $base {
				<$atom$(<$t>)?>::get_mut(self)
			}

			#[inline]
			fn into_inner(self) -> $base {
				<$atom$(<$t>)?>::into_inner(self)
			}

			#[inline]
			fn load(&self, order: Ordering) -> $base {
				<$atom$(<$t>)?>::load(self, order)
			}

			#[inline]
			fn store(&self, value: $base, order: Ordering) {
				<$atom$(<$t>)?>::store(self, value, order);
			}

			#[inline]
			fn swap(&self, value: $base, order: Ordering) -> $base {
				<$atom$(<$t>)?>::swap(self, value, order)
			}

			#[inline]
			fn compare_and_swap(
				&self,
				current: $base,
				new: $base,
				order: Ordering,
			) -> $base {
				match <$atom$(<$t>)?>::compare_exchange_weak(
					self,
					current,
					new,
					order,
					order,
				) {
					Ok(val) => val,
					Err(val) => val,
				}
			}

			#[inline]
			fn compare_exchange(
				&self,
				current: $base,
				new: $base,
				success: Ordering,
				failure: Ordering,
			) -> Result<$base, $base> {
				<$atom$(<$t>)?>::compare_exchange(
					self,
					current,
					new,
					success,
					failure,
				)
			}

			#[inline]
			fn compare_exchange_weak(
				&self,
				current: $base,
				new: $base,
				success: Ordering,
				failure: Ordering,
			) -> Result<$base, $base> {
				<$atom$(<$t>)?>::compare_exchange_weak(
					self,
					current,
					new,
					success,
					failure,
				)
			}

			portable!($bit $(@<$t>)? $atom => $base);

			portable!($num $(@<$t>)? $atom => $base);

			#[inline]
			fn fetch_update<F>(
				&self,
				set_order: Ordering,
				fetch_order: Ordering,
				mut func: F,
			) -> Result<$base, $base>
			where
				F: FnMut($base) -> Option<$base>,
			{
				loop {
					let old = <$atom$(<$t>)?>::load(self, fetch_order);
					let new = func(old).ok_or(old)?;
					match <$atom$(<$t>)?>::compare_exchange_weak(
						self,
						old,
						new,
						set_order,
						fetch_order,
					) {
						Ok(val) => return Ok(val),
						Err(_) => continue,
					}
				}
			}
		}
	)+ )+ };

	(bit $(@<$t:ident>)? $atom:ident => $base:ty) => {
		#[inline]
		fn fetch_and(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_and(self, value, order)
		}

		#[inline]
		fn fetch_nand(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_nand(self, value, order)
		}

		#[inline]
		fn fetch_or(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_or(self, value, order)
		}

		#[inline]
		fn fetch_xor(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_xor(self, value, order)
		}
	};

	(num $(@<$t:ident>)? $atom:ident => $base:ty) => {
		#[inline]
		fn fetch_add(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_add(self, value, order)
		}

		#[inline]
		fn fetch_sub(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_sub(self, value, order)
		}

		#[inline]
		fn fetch_max(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_max(self, value, order)
		}

		#[inline]
		fn fetch_min(&self, value: $base, order: Ordering) -> $base {
			<$atom$(<$t>)?>::fetch_min(self, value, order)
		}
	};

	(no_bit $(@<$t:ident>)? $atom:ident => $base:ty) => {
		portable!(unreachable fetch_and, fetch_nand, fetch_or, fetch_xor);
	};
	(no_num $(@<$t:ident>)? $atom:ident => $base:ty) => {
		portable!(unreachable fetch_add, fetch_sub, fetch_max, fetch_min);
	};

	(unreachable $($n:ident),+ $(,)?) => { $(
		fn $n(&self, _: Self::Item, _: Ordering) -> Self::Item {
			unreachable!(
				"This function is statically guaranteed to never be callable",
			);
		}
	)+ };
}

portable! {
	"8": bit no_num => {
		bool => AtomicBool;
	}
	"8": bit num => {
		i8 => AtomicI8;
		u8 => AtomicU8;
	}
	"16": bit num => {
		i16 => AtomicI16;
		u16 => AtomicU16;
	}
	"32": bit num => {
		i32 => AtomicI32;
		u32 => AtomicU32;
	}
	"64": bit num => {
		i64 => AtomicI64;
		u64 => AtomicU64;
	}
	"128": bit num => {
		i128 => AtomicI128;
		u128 => AtomicU128;
	}
	"ptr": bit num => {
		isize => AtomicIsize;
		usize => AtomicUsize;
	}
	"ptr": no_bit no_num => {
		@<T> *mut T => AtomicPtr;
	}
}
