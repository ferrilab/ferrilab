//! Codegen helpers

/// Produces a doc-string that forwards to a standard library item.
macro_rules! doc_url {
	(const $t:ty => $c:ident) => {
		concat!(
			"See <https://doc.rust-lang.org/std/primitive.",
			stringify!($t),
			".html#associatedconstant.",
			stringify!($c),
			">."
		)
	};
	(mod const $t:ty => $c:ident) => {
		concat!(
			"See <https://doc.rust-lang.org/std/",
			stringify!($t),
			"/consts/constant.",
			stringify!($c),
			".html>."
		)
	};
	(fn $t:ty => $f:ident) => {
		concat!(
			"See <https://doc.rust-lang.org/std/primitive.",
			stringify!($t),
			".html#method.",
			stringify!($f),
			">.",
		)
	};
}

/// Produces a trait definition whose items are linked to their corresponding
/// `i32` API.
macro_rules! new_trait {
	// The outer trait declaration, including all its requirements.
	//
	// Note: due to quirks of Rust's macro rules, trait requirements cannot use
	// their standard `+` separator, and HRTBs need decoration.
	(
		$(#[$attr:meta])*
		$name:ident
		$(: $req:path $(, $(@for<$lt:lifetime>)? $reqs:path)*)?
		{
			$($rest:tt)*
		}
	) => {
		$(#[$attr])*
		pub trait $name
		$(: $req $(+ $(for<$lt>)? $reqs)*)?
		{
			$($rest)*
		}
	};

	// Associated types.
	($($(#[$attr:meta])* type $name:ident $(: $req:path)?;)+) => { $(
		$(#[$attr])* type $name$(: $req)?;
	)+ };

	// Constants that do not exist in the standard library.
	($($(#[$attr:meta])* @new const $name:ident : $type:ty;)+ ) => { $(
		$(#[$attr])* const $name: $type;
	)+ };
	// Freestanding constants.
	($basis:ident @ $(
		$(#[$attr:meta])*
		mod const $name:ident: $type:ty;
	)+) => { $(
		$(#[$attr:meta])*
		#[doc = doc_url!(mod const $basis => $name)]
		const $name: $type;
	)+ };
	// Associated constants.
	($basis:ident @ $(
		$(#[$attr:meta])*
		const $name:ident: $type:ty;
	)+) => { $(
		$(#[$attr])*
		#[doc = doc_url!(const $basis => $name)]
		const $name: $type;
	)+ };

	// Functions.
	($basis:ident @ $(
		$(#[$attr:meta])*
		fn $name:ident
		($($args:tt)*)
		$(-> $ret:ty)?;
	)+) => { $(
		#[doc = doc_url!(fn $basis => $name)]
		$(#[$attr])*
		fn $name ($($args)*) $(-> $ret)?;
	)+ };
}

/// Creates new wrapper items that forward to the corresponding items in the
/// standard library.
///
/// This macro can accept multiple item declarations in a single invocation, but
/// all of its contents *must* be of the same form. Different forms require
/// different invocations.
macro_rules! items {
	// Associated constants.
	($typ:ty => $(const $name:ident: $t:ty;)+) => { $(
		#[doc = doc_url!(const $typ => $name)]
		const $name: $t = <$typ>::$name;
	)+ };
	// Freestanding constants.
	($typ:ident => $(mod const $name:ident: $t:ty;)+) => { $(
		#[doc = doc_url!(mod const $typ => $name)]
		const $name: $t = core::$typ::consts::$name;
	)+ };

	// Methods that take `self` by value.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		(self$(, $arg:ident: $t:ty)*)
		$(-> $ret:ty)?;
	)+ ) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name(self$(, $arg: $t)*) $(-> $ret)? {
			<Self>::$name(self$(, $arg)*)
		}
	)+ };
	// Methods that take `&self` by reference.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		(&self$(, $arg:ident: $t:ty)*)
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name(&self$(, $arg: $t)*) $(-> $ret)? {
			<Self>::$name(&self$(, $arg )*)
		}
	)+ };
	// Methods that take `&mut self` by mutable reference.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		(&mut self$(, $arg:ident: $t:ty)*)
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name(&mut self$(, $arg: $t)*) $(-> $ret)? {
			<Self>::$name(&mut self$(, $arg)*)
		}
	)+ };
	// Functions that do not take `self` at all.
	($typ:ty => $(
		$(#[$attr:meta])*
		fn $name:ident
		($($arg:ident: $t:ty),* $(,)?)
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		#[doc = doc_url!(fn $typ => $name)]
		fn $name($($arg: $t),*) $(-> $ret)? {
			<Self>::$name($($arg),*)
		}
	)+ };
}
