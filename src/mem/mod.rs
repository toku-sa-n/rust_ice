// SPDX-License-Identifier: GPL-3.0-or-later

pub mod accessor;

use core::marker::PhantomData;

pub struct Accessor<T: ?Sized> {
    _marker: PhantomData<T>,
}
impl<T> Accessor<T> {
    pub fn read(&self) -> T {}
}
