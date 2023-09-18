use std::cell::{RefCell, RefMut};
use std::mem;
use std::rc::Rc;

use crate::error::Error;
use crate::Emit;
use crate::FixupKind;


struct Fixup<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    label_id: usize,
    fixup: crate::Fixup<E, F>,
}

pub struct LabelBuffer<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    labels: Vec<Option<u64>>,
    fixups: Vec<Fixup<E, F>>,
}

impl<E, F> Drop for LabelBuffer<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    fn drop(&mut self) {
        debug_assert!(self.fixups.is_empty());
    }
}

impl<E, F> LabelBuffer<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn new() -> LabelBuffer<E, F> {
        LabelBuffer {
            labels: Vec::new(),
            fixups: Vec::new(),
        }
    }

    #[inline]
    pub fn new_label_id(&mut self) -> LabelId {
        let id = self.labels.len();
        self.labels.push(None);
        LabelId { id }
    }

    #[inline]
    pub fn new_label(&mut self) -> Label<'_, E, F> {
        let id = self.labels.len();
        self.labels.push(None);
        Label {
            buffer: self,
            id,
        }
    }

    #[inline]
    pub fn label(&mut self, id: LabelId) -> Label<'_, E, F> {
        Label {
            buffer: self,
            id: id.id,
        }
    }

    #[inline]
    pub fn fixup_all(&mut self, emit: &mut E) -> Result<(), Error> {
        let fixups = mem::replace(&mut self.fixups, Vec::new());
        for fixup in fixups.into_iter() {
            let label_position = self.labels[fixup.label_id].unwrap();
            fixup.fixup.apply_fixup(emit, label_position)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct LabelId {
    id: usize,
}

pub struct Label<'a, E, F>
    where E: Emit,
          F: FixupKind<E>
{
    buffer: &'a mut LabelBuffer<E, F>,
    id: usize,
}

impl<E, F> crate::Label<E, F> for Label<'_, E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        self.buffer.labels[self.id]
    }

    #[inline]
    fn add_fixup(&mut self, fixup: crate::Fixup<E, F>) {
        self.buffer.fixups.push(Fixup {
            label_id: self.id,
            fixup,
        });
    }

    #[inline]
    fn bind(&mut self, _e: &mut E, position: u64) -> Result<(), Error> {
        debug_assert!(self.position().is_none());
        self.buffer.labels[self.id] = Some(position);
        Ok(())
    }
}


pub struct RcLabelBuffer<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    inner: Rc<RefCell<LabelBuffer<E, F>>>,
}

impl<E, F> RcLabelBuffer<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn new() -> RcLabelBuffer<E, F> {
        RcLabelBuffer {
            inner: Rc::new(RefCell::new(LabelBuffer {
                labels: Vec::new(),
                fixups: Vec::new(),
            }))
        }
    }

    #[inline]
    pub fn new_label(&mut self) -> RcLabel<E, F> {
        let mut inner = self.inner.borrow_mut();
        let id = inner.labels.len();
        inner.labels.push(None);
        RcLabel {
            buffer: self.inner.clone(),
            id,
        }
    }

    #[inline]
    pub fn fixup_all(&mut self, emit: &mut E) -> Result<(), Error> {
        self.inner.borrow_mut().fixup_all(emit)
    }
}

pub struct RcLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    buffer: Rc<RefCell<LabelBuffer<E, F>>>,
    id: usize,
}

impl<E, F> RcLabel<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn borrow_mut(&self) -> RcLabelRefMut<'_, E, F> {
        RcLabelRefMut {
            buffer: self.buffer.borrow_mut(),
            id: self.id,
        }
    }
}

pub struct RcLabelRefMut<'a, E, F>
    where E: Emit,
          F: FixupKind<E>
{
    buffer: RefMut<'a, LabelBuffer<E, F>>,
    id: usize,
}

impl<E, F> crate::Label<E, F> for RcLabelRefMut<'_, E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        self.buffer.labels[self.id]
    }

    #[inline]
    fn add_fixup(&mut self, fixup: crate::Fixup<E, F>) {
        self.buffer.fixups.push(Fixup {
            label_id: self.id,
            fixup,
        });
    }

    #[inline]
    fn bind(&mut self, _e: &mut E, position: u64) -> Result<(), Error> {
        debug_assert!(self.position().is_none());
        self.buffer.labels[self.id] = Some(position);
        Ok(())
    }
}
