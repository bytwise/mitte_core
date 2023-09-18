use std::mem;

use vec_map::VecMap;

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

pub struct LabelMap<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    labels: VecMap<u64>,
    fixups: Vec<Fixup<E, F>>,
}

impl<E, F> Drop for LabelMap<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    fn drop(&mut self) {
        debug_assert!(self.fixups.is_empty());
    }
}

impl<E, F> LabelMap<E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    pub fn new() -> LabelMap<E, F> {
        LabelMap {
            labels: VecMap::new(),
            fixups: Vec::new(),
        }
    }

    #[inline]
    pub fn label(&mut self, id: usize) -> Label<'_, E, F> {
        Label {
            map: self,
            id: id,
        }
    }

    #[inline]
    pub fn fixup_all(&mut self, emit: &mut E) -> Result<(), Error> {
        let fixups = mem::replace(&mut self.fixups, Vec::new());
        for fixup in fixups.into_iter() {
            let label_position = self.labels[fixup.label_id];
            fixup.fixup.apply_fixup(emit, label_position)?;
        }
        Ok(())
    }
}

pub struct Label<'a, E, F>
    where E: Emit,
          F: FixupKind<E>
{
    map: &'a mut LabelMap<E, F>,
    id: usize,
}

impl<E, F> crate::Label<E, F> for Label<'_, E, F>
    where E: Emit,
          F: FixupKind<E>
{
    #[inline]
    fn position(&self) -> Option<u64> {
        None
    }

    #[inline]
    fn add_fixup(&mut self, fixup: crate::Fixup<E, F>) {
        self.map.fixups.push(Fixup {
            label_id: self.id,
            fixup,
        });
    }

    #[inline]
    fn bind(&mut self, _e: &mut E, position: u64) -> Result<(), Error> {
        debug_assert!(self.map.labels.get(self.id).is_none());
        self.map.labels.insert(self.id, position);
        Ok(())
    }
}
