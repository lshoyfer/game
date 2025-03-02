use std::hash::Hash;

use crate::prelude::*;

// This absolute garbage purely exists for my own fun, as it truly doesn't matter if Option<usize> takes that extra byte, there are no mem or performance issues.
// (No need to keep it generic as we only use usize in this context and knowing that allows us to use bitwise/native operators to speed up comparison without dipping into more heavy duty ideas (since we are cross-compiling)).
/// A rudimentary option type that uses a 1's bit pattern of usize instead of 0 for NPO/niche optimization. 
#[derive(Clone, Copy)]
pub(super) struct CustomUsizeOption(MaybeUninit<usize>); // Essentially a wrapper type for MaybeUninit as a hack to save memory while allowing a 0 bit pattern
impl CustomUsizeOption {
    pub(super) fn none() -> CustomUsizeOption {
        let mut b = MaybeUninit::uninit();
        b.write(usize::MAX);
        CustomUsizeOption(b)
    }

    /// ## Safety
    /// Caller must ensure the input usize does not ever validly hold the usize::MAX value, otherwise is_some() will read as false
    #[inline]
    pub(super) unsafe fn some(val: usize) -> CustomUsizeOption {
        CustomUsizeOption(MaybeUninit::new(val))
    }

    pub(super) fn is_none(&self) -> bool {
        let val = unsafe { self.0.as_ptr().read() };
        val == usize::MAX
    }
    #[inline]
    pub(super) fn is_some(&self) -> bool {
        let val = unsafe { self.0.as_ptr().read() } ;
        val != usize::MAX
    }

    /// ## Invariant
    /// Caller must ensure the option is not in the `None` state, otherwise returned value will be `usize::MAX` and you will incur strange looks from God.
    // ## Safety (only applies to this module as the inner MaybeUninit field is not public -- unless of course someone bypasses it with mut ptrs but that's their problem)
    // Caller must ensure interior memory was initialized with the [`CustomUsizeOption::some()`] or [`CustomUsizeOption::none()`] functions
    // prior to calling this function for memory safety.
    pub(super) unsafe fn unwrap_unchecked(self) -> usize {
        self.0.assume_init()
    }
}

impl PartialEq for CustomUsizeOption {
    fn eq(&self, other: &Self) -> bool {
        // Safety: Outside of this module, the MaybeUninit can only be initalized with technically valid bit patterns for usize that work for comparison 
        // operations, even if usize::MAX was erroneously deposited into the unsafe some() function and therefore the invariant was violated, this is always valid.
        // It is only undefined behavior/unsafe if the inner MaybeUninit is able to be left uninitialized.
        unsafe {
            self.0.as_ptr().read() == other.0.as_ptr().read()
        }
    }
}

impl Eq for CustomUsizeOption {}

impl Hash for CustomUsizeOption {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Safety: Outside of this module, the MaybeUninit can only be initalized with technically valid bit patterns for usize that work for comparison 
        // operations, even if usize::MAX was erroneously deposited into the unsafe some() function and therefore the invariant was violated, this is always valid
        // It is only undefined behavior/unsafe if the inner MaybeUninit is able to be left uninitialized.
        unsafe {
            self.0.as_ptr().read().hash(state)
        }
    }
}

impl Debug for CustomUsizeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_some() {
            write!(f, "CustomUsizeOption(Some({}))", unsafe { self.0.as_ptr().read() })
        } else {
            write!(f, "CustomUsizeOption(None)")
        }
    }
}