use std::io::{self, Cursor, Write};
use std::ops::Range;
use std::convert::TryInto;

use crate::error::Error;
use crate::{Emit, EmitSlice};


impl EmitSlice for Vec<u8> {
    type Error = std::convert::Infallible;

    #[inline]
    fn emit_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.extend_from_slice(slice);
        Ok(())
    }
}

impl Emit for Vec<u8> {
    #[inline]
    fn position(&self) -> u64 {
        self.len() as u64
    }

    #[inline]
    fn get_mut_slice(&mut self, range: Range<u64>) -> Result<&mut [u8], Error> {
        self.get_mut(range.start as usize..range.end as usize)
            .ok_or(Error::OutOfBounds)
    }

    #[inline]
    fn get_mut_array<const N: usize>(&mut self, position: u64) -> Result<&mut [u8; N], Error> {
        let slice = self.get_mut((position as usize)..(position as usize + N));
        slice.and_then(|slice| slice.try_into().ok())
            .ok_or(Error::OutOfBounds)
    }
}


impl<W> EmitSlice for Cursor<W>
    where Cursor<W>: Write
{
    type Error = io::Error;

    #[inline]
    fn emit_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error> {
        self.write_all(slice)
    }
}

impl<W> Emit for Cursor<W>
    where Cursor<W>: Write, W: AsMut<[u8]>
{
    #[inline]
    fn position(&self) -> u64 {
        self.position()
    }

    #[inline]
    fn get_mut_slice(&mut self, range: Range<u64>) -> Result<&mut [u8], Error> {
        self.get_mut().as_mut().get_mut(range.start as usize..range.end as usize)
            .ok_or(Error::OutOfBounds)
    }

    #[inline]
    fn get_mut_array<const N: usize>(&mut self, position: u64) -> Result<&mut [u8; N], Error> {
        let slice = self.get_mut().as_mut().get_mut((position as usize)..(position as usize + N));
        slice.and_then(|slice| slice.try_into().ok())
            .ok_or(Error::OutOfBounds)
    }
}
