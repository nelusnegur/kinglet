use core::ops::Deref;
use core::ptr::NonNull;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Sr<T> {
    ptr: NonNull<T>
}

impl<T> Sr<T> {
    /// Creates a static reference from a raw pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null, aligned and valid 
    /// for the entire execution of the program.
    pub const unsafe fn new(ptr: *mut T) -> Sr<T> {
        Sr {
            ptr: NonNull::new_unchecked(ptr)
        }
    }
}

impl<T> Deref for Sr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY
        // As promised by the caller of `Sr::new`, the ptr is non-null, 
        // aligned and valid for the entire execution of the program.
        unsafe { self.ptr.as_ref() }
    }
}
