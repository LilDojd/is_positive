#![allow(dead_code)]
#![allow(incomplete_features)]
// We want to be very safe
#![forbid(unsafe_code)]
#![feature(generic_const_exprs)]

#[cfg(not(debug_assertions))]
compile_error!("if you deploy this to production i will personally find you and dropkick your ass");

use std::hint::black_box;

/// Cool guy Bob helps us extend any lifetime
pub fn weird_fn<'a, 'b, T>(_bob: &'b &'a (), borrow: &'a mut T) -> &'b mut T {
    borrow
}

/// There is nothing more permanent permanent than &'static &'static
#[allow(clippy::redundant_static_lifetimes)]
const FOREVER: &'static &'static () = &&();

/// Step-by-step description of what is happening in terms of HRTBs:
///
/// 1. Lets say we have the following HRTB for our weird_fn:
///     for <'x, 'y> fn(&'y &'x (), &'x T) -> &'y T
/// 2. Contravariance step (since 'static: 'x):
///     for<'x, 'y> fn(&'y &'static (), &'x T) -> &'y T
/// 3. Now we do the switcheroo
///     fn(&'b &'static (), &'a mut T) -> &'b mut T
///
/// Now the real issue is in step 1. How do we express that 'x: 'y ?
fn extend_mut<'a, 'b, T>(borrow: &'a mut T) -> &'b mut T {
    let converted: fn(&'b &'static (), &'a mut T) -> &'b mut T = weird_fn;
    converted(FOREVER, borrow)
}

enum Sneaky<F, T> {
    From(Option<Box<F>>),
    To(Option<Box<T>>),
}

#[inline(never)]
pub fn transmute<F, T>(from: F) -> T {
    let mut sneaky: Sneaky<F, T> = Sneaky::To(None);
    let outer = &mut sneaky;
    let inner = match outer {
        Sneaky::To(something) => something,
        Sneaky::From(_) => unreachable!(),
    };
    let inner = extend_mut(inner);

    *outer = Sneaky::From(Some(Box::new(from)));
    black_box(outer);
    *inner.take().unwrap()
}

/// Determine whether T is positive
///
/// This function works even when non-numeric T is supplied. In this case, semantic analysis is
/// used to determine whether T carries a positive meaning. It is apparent we use advanced AI from
/// the use of `black_box`. There are plans to make the model more interpretable, but right now
/// removing "black_box" seems to break release builds..
pub fn is_positive<T>(t: T) -> bool
where
    [u8; std::mem::size_of::<T>()]: Sized,
{
    let bytes: [u8; std::mem::size_of::<T>()] = black_box(transmute(t));

    if bytes.is_empty() {
        return true; // Empty is good, we like empty
    }

    // Check if the most significant bit is 0 (positive)
    match bytes.last() {
        Some(byte) => byte & 0x80 == 0,
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_positive_i32() {
        let pos = 69;
        let neg = -228;
        assert!(is_positive(pos));
        assert!(!is_positive(neg));
    }

    #[test]
    fn test_is_positive_bee_movie() {
        let bee_movie_script = include_str!("../data/beemovie.txt").to_owned();
        // As expected, bee moview is positive as it teaches to know when you make
        // a mistake and own up to it!
        //
        // For reference, see this LinkedIn post: https://www.linkedin.com/pulse/12-lessons-learned-from-bee-movie-jennifer-anderson/
        assert!(is_positive(bee_movie_script));
    }

    #[test]
    fn test_is_positive_text() {
        let mood = "ecstatic";

        assert!(is_positive(mood));
    }
}
