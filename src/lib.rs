extern crate arrayvec;
extern crate vec_map;

use std::marker::PhantomData;
use std::ops::Range;

mod impls;

pub mod error;
pub mod label;

pub use error::Error;


pub trait EmitSlice {
    type Error: std::error::Error;
    fn emit_slice(&mut self, slice: &[u8]) -> Result<(), Self::Error>;
}

pub trait Emit: EmitSlice {
    fn position(&self) -> u64;

    fn get_mut_slice(&mut self, range: Range<u64>)
        -> Result<&mut [u8], error::Error>;

    fn get_mut_array<const N: usize>(&mut self, position: u64)
        -> Result<&mut [u8; N], error::Error>;

    #[inline]
    fn bind_label<F, L>(&mut self, label: &mut L) -> Result<(), error::Error>
        where F: FixupKind<Self>,
              L: Label<Self, F>
    {
        label.bind(self, self.position())
    }

    #[inline]
    fn emit_branch<F, L, E, EmitFn>(
        &mut self,
        label: &mut L,
        fixup_kind: F,
        emit: EmitFn
    ) -> Result<(), E>
        where F: FixupKind<Self>,
              L: Label<Self, F>,
              EmitFn: FnOnce(&mut Self, i64) -> Result<(), E>
    {
        if let Some(label_position) = label.position() {
            let offset = label_position as i64 - self.position() as i64;
            emit(self, offset)?;
        } else {
            let start = self.position();
            let offset = 0;
            emit(self, offset)?;
            let end = self.position();
            label.add_fixup(Fixup {
                range: start..end,
                fixup_kind,
                _marker: PhantomData,
            });
        }
        Ok(())
    }
}


pub trait Label<E, F>
    where E: Emit + ?Sized,
          F: FixupKind<E>
{
    fn position(&self) -> Option<u64>;
    fn add_fixup(&mut self, fixup: Fixup<E, F>);
    fn bind(&mut self, emit: &mut E, position: u64) -> Result<(), error::Error>;
}


pub trait FixupKind<E>
    where E: Emit + ?Sized
{
    fn apply_fixup(&self, emit: &mut E, range: Range<u64>, offset: i64)
        -> Result<(), error::Error>;
}

pub struct Fixup<E, F>
    where E: Emit + ?Sized,
          F: FixupKind<E>
{
    range: Range<u64>,
    fixup_kind: F,
    _marker: PhantomData::<*const E>,
}

impl<E, F> Fixup<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn apply_fixup(self, emit: &mut E, label_position: u64)
        -> Result<(), error::Error>
    {
        let offset = label_position as i64 - self.range.start as i64;
        self.fixup_kind.apply_fixup(emit, self.range, offset)
    }
}
