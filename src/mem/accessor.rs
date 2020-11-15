use core::marker::PhantomData;

pub struct Accessor<T: ?Sized> {
    _marker: PhantomData<T>,
}
impl<T> Accessor<T> {
    pub fn read(&self) -> T {}
}
