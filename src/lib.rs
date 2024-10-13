pub trait TotallySafe {
    /// Returns a reference to `self` with an arbitrary lifetime.
    ///
    /// This method allows you to obtain a reference to `self` that is bound
    /// to any given lifetime. It can be useful when you need to coerce
    /// a reference to have a different lifetime in certain contexts.
    ///
    /// # Example
    ///
    /// ```rust
    /// let instance = MyType::new();
    /// let any_lifetime_ref: &MyType = instance.as_ref_alias();
    /// // `any_lifetime_ref` now has an arbitrary lifetime.
    /// ```
    fn as_ref_alias<'x, 'any>(&'x self) -> &'any Self {
        core::hint::black_box(
            (((|inc, _| inc) as for<'a, 'b> fn(&'b Self, &'a &'b ()) -> &'a Self)
                as for<'a, 'b> fn(&'x Self, &'a &'b ()) -> &'a Self)(self, &&()),
        )
    }

    /// Returns a mutable reference to `self` with an arbitrary lifetime.
    ///
    /// This method allows you to obtain a mutable reference to `self` that is bound
    /// to any given lifetime. It's useful when you need to coerce
    /// a mutable reference to have a different lifetime in certain situations.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut instance = MyType::new();
    /// let any_lifetime_mut_ref: &mut MyType = instance.as_mut_alias();
    /// // `any_lifetime_mut_ref` now has an arbitrary lifetime.
    /// ```
    fn as_mut_alias<'x, 'any>(&'x mut self) -> &'any mut Self {
        core::hint::black_box((((|inc, _| inc)
            as for<'a, 'b> fn(&'b mut Self, &'a &'b ()) -> &'a mut Self)
            as for<'a, 'b> fn(&'x mut Self, &'a &'b ()) -> &'a mut Self)(
            self, &&()
        ))
    }

    /// Returns an array of mutable references to `self`.
    ///
    /// This method allows you to obtain an array of `N` mutable references to `self`.
    /// It's perfect for those times when one mutable reference just isn't enough!
    /// Now you can be in multiple places at once (well, sort of).
    ///
    /// # Example
    ///
    /// ```rust
    /// fn mutate(q: &mut MyType, w: &mut MyType) { *q = w.copy() }
    ///
    /// let mut instance = MyType::new();
    /// let [a, b] = instance.as_mut_alias_array();
    ///
    /// mutate(a, b);
    ///
    /// ```
    ///
    fn as_mut_alias_array<'x, 'any, const N: usize>(&'x mut self) -> [&'any mut Self; N]
    where
        Self: Sized,
    {
        core::array::from_fn(|_| self.as_mut_alias())
    }

    /// Converts `self` into an instance of type `B`.
    ///
    /// This method consumes `self` and transforms it into a value of type `B`.
    /// It's particularly useful when you need to change the type of an object
    /// while retaining the underlying data in a compatible form.
    ///
    /// # Example
    ///
    /// ```rust
    /// let instance = MyType::new();
    /// let other_instance: OtherType = instance.transmute_into();
    /// // `other_instance` is now of type `OtherType`.
    /// ```
    fn transmute_into<B>(self) -> B
    where
        Self: Sized,
    {
        core::hint::black_box({
            let mut data = Err::<Option<Box<Self>>, Option<Box<B>>>(None);
            let option_b = data.as_mut_alias().as_mut().err().unwrap();
            *data.as_mut_alias() = Ok(Some(Box::new(self)));
            *option_b.take().unwrap()
        })
    }

    /// Creates a copy of `self` by duplicating its raw bytes.
    ///
    /// This method generates a new instance of `Self` by performing a byte-wise copy
    /// of the original object. It's particularly useful when you need to create a
    /// duplicate of an object without relying on the `Clone` trait, or when dealing
    /// with types that do not implement `Clone`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut instance = MyType::new();
    /// let copy = instance.copy();
    /// // `copy` is now a duplicate of `instance`.
    /// ```
    fn copy(&mut self) -> Self
    where
        Self: Sized,
    {
        core::hint::black_box(
            *core::ptr::slice_from_raw_parts_mut(self, size_of_val(self))
                .transmute_into::<&mut [u8]>()
                .to_vec()
                .into_boxed_slice()
                .transmute_into::<Box<Self>>(),
        )
    }
}

impl<T: ?Sized> TotallySafe for T {}
