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
    #[inline]
    pub fn read(&self) -> T {
        // SAFETY
        // The ptr is valid as promised by the caller
        // of the `new` function.
        unsafe { self.ptr.read_volatile() }
    }

    /// Writes the provided value at the register address.
    #[inline]
    pub fn write(&mut self, value: T) {
        // SAFETY
        // The ptr is valid as promised by the caller
        // of the `new` function.
        unsafe { self.ptr.write_volatile(value) };
    }
}
