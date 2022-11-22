//! This is a small example which demonstrates how a function which does not
//! care about atomicity can be used to operate on a value.

use std::{
	cell::Cell,
	sync::atomic::{
		AtomicU64,
		Ordering,
	},
	thread,
	time::Duration,
};

use radium::{
	types::{
		Atom,
		Isotope,
		RadiumU64,
		Radon,
	},
	Radium,
};

fn do_work<R: Radium<Item = u64>>(this: &R, ident: u8) {
	let on_entry = this.load(Ordering::SeqCst);
	println!("{: >2} step 0 sees: {: >2}", ident, on_entry);

	let before_add = this.fetch_add(10, Ordering::SeqCst);
	println!("{: >2} step 1 sees: {: >2}", ident, before_add);

	let after_add = this.load(Ordering::SeqCst);
	println!("{: >2} step 2 sees: {: >2}", ident, after_add);

	thread::sleep(Duration::from_millis(after_add));

	let before_sub = this.fetch_sub(3, Ordering::SeqCst);
	println!("{: >2} step 3 sees: {: >2}", ident, before_sub);

	let on_exit = this.load(Ordering::SeqCst);
	println!("{: >2} step 4 sees: {: >2}", ident, on_exit);
}

fn run_thrice<R: Radium<Item = u64> + Sync>(item: &'static R, ident: u8) {
	for th in
		(ident .. (ident + 3)).map(|id| thread::spawn(move || do_work(item, id)))
	{
		let _ = th.join();
	}
}

static ATOM: AtomicU64 = AtomicU64::new(0);
static RADIUM: RadiumU64 = RadiumU64::new(0);

fn main() {
	let cell = Cell::new(0u64);

	let atom = Atom::new(0u64);
	let isotope = Isotope::new(0u64);
	let radon = Radon::new(0u64);

	println!("atoms");
	run_thrice(&ATOM, 0);
	println!();
	let atom = Box::leak(Box::new(atom));
	run_thrice(atom, 3);
	println!();

	println!("isotopes");
	run_thrice(&RADIUM, 6);
	println!();
	for ident in 9 .. 12 {
		do_work(&isotope, ident);
	}
	println!();

	println!("cells");
	for ident in 12 .. 15 {
		do_work(&cell, ident);
	}
	println!();
	for ident in 15 .. 18 {
		do_work(&radon, ident);
	}
	println!();
}
