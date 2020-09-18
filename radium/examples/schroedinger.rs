//! This is a small example which demonstrates how a function which does not
//! care about atomicity can be used to operate on a value.
//!
//! The context which *uses* such a function must still care about atomicity;
//! `radium` does not suddenly permit `Cell` to cross threads. It just provides
//! a unified trait interface for a cell and an atom of the same underlying
//! type.

use radium::Radium;

use std::{
    cell::Cell,
    sync::atomic::{AtomicU32, Ordering},
    thread,
    time::Duration,
};

/// Operates on a value, which might or might not be atomic.
fn routine<R: Radium<Item = u32>>(obj: &R, ident: usize) {
    println!(
        "Entry {} observes value: {}",
        ident,
        obj.load(Ordering::Relaxed)
    );
    let added = obj.fetch_add(1, Ordering::Relaxed);
    println!("Middle {} observes fetched value: {}", ident, added);
    println!(
        "Middle {} observes loaded value:  {}",
        ident,
        obj.load(Ordering::Relaxed)
    );
    thread::sleep(Duration::from_millis(
        obj.load(Ordering::Relaxed) as u64 * 10,
    ));
    let subbed = obj.fetch_sub(1, Ordering::Relaxed);
    println!("Exit {} observes fetched value: {}", ident, subbed);
    println!(
        "Exit {} observes loaded value:  {}",
        ident,
        obj.load(Ordering::Relaxed)
    );
}

/// Single value which will be contended by multiple threads
static HOT: AtomicU32 = AtomicU32::new(0);

fn main() {
    //  This cannot cross a thread, so it is only accessed without contention in
    //  an ordered call sequence.
    let cold = Cell::new(0u32);

    routine(&cold, 0);
    let one = thread::spawn(move || {
        routine(&HOT, 1);
    });
    let two = thread::spawn(move || {
        routine(&HOT, 2);
    });
    routine(&cold, 3);

    let _ = one.join();
    let _ = two.join();

    assert_eq!(HOT.load(Ordering::Relaxed), 0);
    assert_eq!(cold.load(Ordering::Relaxed), 0);
}
