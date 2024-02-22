use arrayvec::ArrayVec;

use crate::error::Error;
use crate::Emit;
use crate::{FixupKind, Fixup, Label};

pub mod map;
pub mod buffer;

pub use map::LabelMap;
pub use buffer::{LabelBuffer, RcLabelBuffer};


#[derive(Clone, Copy)]
pub struct StaticLabel {
    position: u64,
}

impl StaticLabel {
    #[inline]
    pub fn from_position(position: u64) -> StaticLabel {
        StaticLabel {
            position,
        }
    }
}

impl<E, F> Label<E, F> for StaticLabel
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        Some(self.position)
    }

    #[inline]
    fn add_fixup(&mut self, _fixup: Fixup<E, F>) {
        panic!("static labels don't need fixups");
    }

    #[inline]
    fn bind(&mut self, _emit: &mut E, _position: u64) -> Result<(), Error> {
        panic!("label already bound");
    }
}


pub struct OptionLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    position: Option<u64>,
    fixup: Option<Fixup<E, F>>,
}

impl<E, F> OptionLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn new() -> OptionLabel<E, F> {
        OptionLabel {
            position: None,
            fixup: None,
        }
    }
}

impl<E, F> Label<E, F> for OptionLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        self.position
    }

    #[inline]
    fn add_fixup(&mut self, fixup: Fixup<E, F>) {
        assert!(self.fixup.is_none());
        self.fixup = Some(fixup);
    }

    #[inline]
    fn bind(&mut self, emit: &mut E, position: u64) -> Result<(), Error> {
        debug_assert!(self.position.is_none());
        self.position = Some(position);
        if let Some(fixup) = self.fixup.take() {
            fixup.apply_fixup(emit, position)?;
        }
        Ok(())
    }
}


pub struct VecLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    position: Option<u64>,
    fixups: Vec<Fixup<E, F>>,
}

impl<E, F> VecLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn new() -> VecLabel<E, F> {
        VecLabel {
            position: None,
            fixups: Vec::new(),
        }
    }
}

impl<E, F> Label<E, F> for VecLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        self.position
    }

    #[inline]
    fn add_fixup(&mut self, fixup: Fixup<E, F>) {
        self.fixups.push(fixup);
    }

    #[inline]
    fn bind(&mut self, emit: &mut E, position: u64) -> Result<(), Error> {
        debug_assert!(self.position.is_none());
        self.position = Some(position);
        for fixup in self.fixups.drain(..) {
            fixup.apply_fixup(emit, position)?;
        }
        Ok(())
    }
}


pub struct ArrayVecLabel<E, F, const N: usize>
    where E: Emit,
          F: FixupKind<E>
{
    position: Option<u64>,
    fixups: ArrayVec<Fixup<E, F>, N>,
}

impl<E, F, const N: usize> ArrayVecLabel<E, F, N>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn new() -> ArrayVecLabel<E, F, N> {
        ArrayVecLabel {
            position: None,
            fixups: ArrayVec::new(),
        }
    }
}

impl<E, F, const N: usize> Label<E, F> for ArrayVecLabel<E, F, N>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        self.position
    }

    #[inline]
    fn add_fixup(&mut self, fixup: Fixup<E, F>) {
        self.fixups.push(fixup);
    }

    #[inline]
    fn bind(&mut self, emit: &mut E, position: u64) -> Result<(), Error> {
        debug_assert!(self.position.is_none());
        self.position = Some(position);
        for fixup in self.fixups.drain(..) {
            fixup.apply_fixup(emit, position)?;
        }
        Ok(())
    }
}
