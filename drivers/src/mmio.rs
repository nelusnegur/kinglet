use core::ptr;
use core::ptr::NonNull;

/// Represents a statically known mutable memory address used for MMIO.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Register<T> {
    ptr: NonNull<T>,
}

impl<T> Register<T> {
    /// Creates a register from a raw pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null, aligned and valid
    /// for the entire execution of the program.
    pub unsafe fn new(ptr: *const T) -> Self {
        Register {
            ptr: NonNull::new_unchecked(ptr.cast_mut()),
        }
    }

    /// Reads the value at the register address.
    #[inline(always)]
    pub fn read(&self) -> T {
        // SAFETY
        // The ptr is valid as promised by the caller
        // of the `new` function.
        unsafe { ptr::read_volatile(self.ptr.as_ptr()) }
    }

    /// Writes the provided value at the register address.
    #[inline(always)]
    pub fn write(&mut self, value: T) {
        // SAFETY
        // The ptr is valid as promised by the caller
        // of the `new` function.
        unsafe { ptr::write_volatile(self.ptr.as_ptr(), value) };
    }
}
