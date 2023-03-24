use std::marker::PhantomData;

#[derive(Clone, PartialEq, Debug)]
pub struct ActionList<'buffer, Buffer: Into<&'buffer [u8]>> {
    pub buffer: Buffer,
    pub buffer_lifetime: PhantomData<&'buffer [u8]>,
}
