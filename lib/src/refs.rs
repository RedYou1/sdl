use std::ops::{Deref, DerefMut};

pub struct Ref<T: ?Sized + 'static> {
    this: *const T,
}
impl<T: ?Sized + 'static> Clone for Ref<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized + 'static> Copy for Ref<T> {}
impl<T: ?Sized + 'static> Ref<T> {
    pub const fn new(this: &T) -> Self {
        Self { this }
    }
    pub const fn as_ref(&self) -> &'static T {
        unsafe { self.this.as_ref_unchecked() }
    }
}
impl<T: ?Sized + 'static> From<&T> for Ref<T> {
    fn from(value: &T) -> Self {
        Self { this: value }
    }
}
impl<T: ?Sized + 'static> From<&mut T> for Ref<T> {
    fn from(value: &mut T) -> Self {
        Self { this: value }
    }
}
impl<T: ?Sized + 'static> From<MutRef<T>> for Ref<T> {
    fn from(value: MutRef<T>) -> Self {
        Self { this: value.this }
    }
}
impl<T: ?Sized + 'static> AsRef<T> for Ref<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.this.as_ref_unchecked() }
    }
}
impl<T: ?Sized + 'static> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.this.as_ref_unchecked() }
    }
}
pub struct MutRef<T: ?Sized + 'static> {
    this: *mut T,
}
impl<T: ?Sized + 'static> Clone for MutRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized + 'static> Copy for MutRef<T> {}
impl<T: ?Sized + 'static> MutRef<T> {
    pub const fn new(this: &mut T) -> Self {
        Self { this }
    }
    pub const fn as_ref(&self) -> &'static T {
        unsafe { self.this.as_ref_unchecked() }
    }
    pub const fn as_mut(&mut self) -> &'static mut T {
        unsafe { self.this.as_mut_unchecked() }
    }
}
impl<T: ?Sized + 'static> AsRef<T> for MutRef<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.this.as_ref_unchecked() }
    }
}
impl<T: ?Sized + 'static> AsMut<T> for MutRef<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.this.as_mut_unchecked() }
    }
}
impl<T: ?Sized + 'static> Deref for MutRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.this.as_ref_unchecked() }
    }
}
impl<T: ?Sized + 'static> DerefMut for MutRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.this.as_mut_unchecked() }
    }
}
impl<T: ?Sized + 'static> From<&mut T> for MutRef<T> {
    fn from(value: &mut T) -> Self {
        Self { this: value }
    }
}
