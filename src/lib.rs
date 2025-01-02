//! An extension trait for [`ControlFlow`] which provides inspection methods
//!
//! The naming conventions are derived from [`Result::inspect_err`]. Since [`ControlFlow`] is more
//! symmetrical than [`Result`], the methods are called [`inspect_break`] and [`inspect_continue`].
//!
//! [`inspect_break`]: ControlFlowExt::inspect_break
//! [`inspect_continue`]: ControlFlowExt::inspect_continue

#![no_std]

use core::ops::ControlFlow;

/// Extension trait for [`ControlFlow`]
pub trait ControlFlowExt {
    /// The break type of [`ControlFlow`]
    type B;

    /// The continue type of [`ControlFlow`]
    type C;

    /// Calls a function with a reference to the contained value if [`Break`].
    ///
    /// Returns the original [`ControlFlow`].
    ///
    /// [`Break`]: ControlFlow::Break
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::ops::ControlFlow;
    /// use controlflow_inspect::ControlFlowExt as _;
    ///
    /// let mut num = 10;
    /// let mut cf = ControlFlow::Break(5);
    /// cf.inspect_break(|n| num += n);
    /// assert_eq!(num, 15);
    ///
    /// cf = ControlFlow::Continue("x");
    /// cf.inspect_break(|n| num += n);
    /// assert_eq!(num, 15);
    /// ```
    fn inspect_break<F: FnOnce(&Self::B)>(self, f: F) -> Self;

    /// Calls a function with a reference to the contained value if [`Continue`].
    ///
    /// Returns the original [`ControlFlow`].
    ///
    /// [`Continue`]: ControlFlow::Continue
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::ops::ControlFlow;
    /// use controlflow_inspect::ControlFlowExt as _;
    ///
    /// let mut num = 10;
    /// let mut cf = ControlFlow::Break("x");
    /// cf.inspect_continue(|n| num += n);
    /// assert_eq!(num, 10);
    ///
    /// cf = ControlFlow::Continue(5);
    /// cf.inspect_continue(|n| num += n);
    /// assert_eq!(num, 15);
    /// ```
    fn inspect_continue<F: FnOnce(&Self::C)>(self, f: F) -> Self;
}

impl<B, C> ControlFlowExt for ControlFlow<B, C> {
    type B = B;
    type C = C;

    fn inspect_break<F: FnOnce(&B)>(self, f: F) -> Self {
        if let Self::Break(ref b) = self {
            f(b);
        }
        self
    }

    fn inspect_continue<F: FnOnce(&C)>(self, f: F) -> Self {
        if let Self::Continue(ref c) = self {
            f(c);
        }
        self
    }
}
