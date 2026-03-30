use core::{cell::Cell};

use crate::{Atom, Isotope, Radium, Radon, marker::{Atomic, Nuclear}};

pub trait RadiumFamily{
    type Me<T> : Radium<Item = T>
    where
        T: Atomic + Nuclear + PartialEq,
        Cell<T>: Radium<Item = T>;
}

pub struct AtomFamily;
impl RadiumFamily for AtomFamily {
    type Me<T> = Atom<T>
        where
            T: Atomic + Nuclear + PartialEq,
            Cell<T>: Radium<Item = T>;
}

pub struct IsotopeFamily;
impl RadiumFamily for IsotopeFamily {
    type Me<T> = Isotope<T>
        where
            T: Atomic + Nuclear + PartialEq,
            Cell<T>: Radium<Item = T>;
}

pub struct RadonFamily;
impl RadiumFamily for RadonFamily {
    type Me<T> = Radon<T>
        where
            T: Atomic + Nuclear + PartialEq,
            Cell<T>: Radium<Item = T>;
}

///This is a shortcut to make the code look more like regular generics.
pub type MaybeAtomic<F, T> = <F as RadiumFamily>::Me<T>;


#[cfg(test)]
mod test {
    use super::*;
    use static_assertions::*;
    use core::{ptr, sync::atomic::Ordering};
    

    type MaybeAtomic<F, T> = <F as RadiumFamily>::Me<T>;

    struct Share<F: RadiumFamily> {

        #[cfg(target_has_atomic = "8")]
        bool: MaybeAtomic<F, bool>,
        #[cfg(target_has_atomic = "8")]
        u8: MaybeAtomic<F, u8>,
        #[cfg(target_has_atomic = "8")]
        i8: MaybeAtomic<F, i8>,

        #[cfg(target_has_atomic = "16")]
        u16: MaybeAtomic<F, u16>,
        #[cfg(target_has_atomic = "16")]
        i16: MaybeAtomic<F, i16>,

        #[cfg(target_has_atomic = "32")]
        u32: MaybeAtomic<F, u32>,
        #[cfg(target_has_atomic = "32")]
        i32: MaybeAtomic<F, i32>,

        #[cfg(target_has_atomic = "64")]
        u64: MaybeAtomic<F, u64>,
        #[cfg(target_has_atomic = "64")]
        i64: MaybeAtomic<F, i64>,

        // 指针和大小类型
        #[cfg(target_has_atomic = "ptr")]
        usize: MaybeAtomic<F, usize>,
        #[cfg(target_has_atomic = "ptr")]
        isize: MaybeAtomic<F, isize>,
        #[cfg(target_has_atomic = "ptr")]
        ptr: MaybeAtomic<F, *mut u8>,
    }

    impl<F: RadiumFamily> Share<F> {
        fn new() -> Self {
            Self {
                bool: Radium::new(false),

                #[cfg(target_has_atomic = "8")]
                u8: Radium::new(0),
                #[cfg(target_has_atomic = "8")]
                i8: Radium::new(0),

                #[cfg(target_has_atomic = "16")]
                u16: Radium::new(0),
                #[cfg(target_has_atomic = "16")]
                i16: Radium::new(0),

                #[cfg(target_has_atomic = "32")]
                u32: Radium::new(0),
                #[cfg(target_has_atomic = "32")]
                i32: Radium::new(0),

                #[cfg(target_has_atomic = "64")]
                u64: Radium::new(0),
                #[cfg(target_has_atomic = "64")]
                i64: Radium::new(0),

                #[cfg(target_has_atomic = "ptr")]
                usize: Radium::new(0),
                #[cfg(target_has_atomic = "ptr")]
                isize: Radium::new(0),
                #[cfg(target_has_atomic = "ptr")]
                ptr: Radium::new(ptr::null_mut()),
            }
        }
    }

    #[test]
    fn test_concurrency_safety() {
        assert_impl_all!(Share<AtomFamily>: Sync);
        assert_not_impl_any!(Share<RadonFamily>: Sync);
    }

    #[test]
    fn test_operation() {
        let s = Share::<RadonFamily>::new();
        
        #[cfg(target_has_atomic = "8")]
        {
        s.bool.store(true, Ordering::Relaxed);
        assert!(s.bool.load(Ordering::Relaxed));
        }
        
        #[cfg(target_has_atomic = "32")]
        {
            s.u32.fetch_add(10, Ordering::Relaxed);
            assert_eq!(s.u32.load(Ordering::Relaxed), 10);
        }
    }
}