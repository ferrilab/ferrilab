/// Helper to route wrapper functions to their underlying impl.
#[macro_export]
macro_rules! pointer_dispatch {
	($(
		$(#[$attr:meta])*
		$(@$comptime:ident)? $(@@$safety:ident)? fn $func:ident
		($(@$this:ident $(,)?)? $($arg:ident : $typ:ty),* $(,)?)
		$(via $rewrap:ident $rewrap_mut:ident)?
		$(-> $ret:ty)?;
	)+) => { $(
		$(#[$attr])*
		#[inline(always)]
		pub $($comptime)? $($safety)? fn $func($($this ,)? $($arg : $typ),*) $(-> $ret)? {
			if P::Original::IS_MUT {
				let out = $($safety)? { <*mut T>::$func($(Pointer::unwrap_mut($this),)? $($arg),*) };
				$(let out = Pointer::$rewrap_mut(out);)?
				out
			}
			else {
				let out = $($safety)? { <*const T>::$func($(Pointer::unwrap($this),)? $($arg),*) };
				$(let out = Pointer::$rewrap(out);)?
				out
			}
		}
	)+ };

	(rewrap $this:ident.$func:ident($($arg:expr),*)) => {
		if P::Original::IS_MUT {
			Pointer::rewrap_mut($this.unwrap_mut().$func($($arg),*))
		}
		else {
			Pointer::rewrap($this.unwrap().$func($($arg),*))
		}
	};

	(nowrap $this:ident.$func:ident($($arg:expr),*)) => {
		if P::Original::IS_MUT {
			$this.unwrap_mut().$func($($arg),*)
		}
		else {
			$this.unwrap().$func($($arg),*)
		}
	};
}
