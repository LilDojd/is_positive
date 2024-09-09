#![allow(dead_code)]
use used_after_drop::extend_mut;

mod used_after_drop;

enum Sneaky<F, T> {
    From(Option<Box<F>>),
    To(Option<Box<T>>),
}

pub fn transmute<F, T>(from: F) -> T {
    let mut sneaky: Sneaky<F, T> = Sneaky::To(None);
    let outer = &mut sneaky;
    let inner = match outer {
        Sneaky::To(something) => something,
        Sneaky::From(_) => unreachable!(),
    };
    let inner = extend_mut(inner);

    *outer = Sneaky::From(Some(Box::new(from)));
    *inner.take().unwrap()
}

/// Make the T whole again (Unbelievable UB levels)
pub fn the_answer<T>(t: &T) {
    struct Byte(u8);

    let f: &mut Byte = transmute(t);
    f.0 = 42;
}

/// Try running tests with release and dev profiles, and observe the UB fuckery
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one_u32() {
        let value: u32 = 69;

        the_answer(&value);

        assert_eq!(value, 42);
    }
}
