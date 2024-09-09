/// Cool guy Bob helps us extend any lifetime to static
pub fn weird_fn<'a, 'b, T>(_bob: &'b &'a (), borrow: &'a mut T) -> &'b mut T {
    borrow
}

/// There is nothing more permanent permanent than &'static &'static
const FOREVER: &&() = &&();

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
pub(crate) fn extend_mut<'a, 'b, T>(borrow: &'a mut T) -> &'b mut T {
    let converted: fn(&'b &'static (), &'a mut T) -> &'b mut T = weird_fn;
    converted(FOREVER, borrow)
}
