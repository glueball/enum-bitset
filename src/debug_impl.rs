use core::{
    fmt,
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

#[doc(hidden)]
pub trait NoDebug {
    #[inline(always)]
    fn debug_entries<T, U>(&self, f: &mut Formatter<'_>, len: usize, _: U) -> fmt::Result
    where
        U: Iterator<Item = T>,
    {
        match len {
            0 => write!(f, "{{}}"),
            1 => write!(f, "{{/* 1 item */}}"),
            n => write!(f, "{{/* {n} items */}}"),
        }
    }
}
impl<T> NoDebug for T {}


pub struct DebugWrapper<T>(pub PhantomData<T>);


impl<T: Debug> DebugWrapper<T> {
    #[inline(always)]
    pub fn debug_entries<U: Iterator<Item = T>>(
        &self,
        f: &mut Formatter<'_>,
        _: usize,
        iter: U,
    ) -> fmt::Result {
        f.debug_set().entries(iter).finish()
    }
}
