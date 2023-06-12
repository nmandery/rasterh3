use ahash::HashSet;
use h3o::{CellIndex, Resolution};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::Error;

/// A container for cells covering an area.
///
/// After calling `CellCoverage::compact()` the contained cells will not overlap, even when they where added
/// in different resolutions. Duplicates will be removed.
///
/// This struct internally uses mostly `sort` instead of `sort_unstable` as the Vec to be sorted are
/// often at least partially sorted.
pub struct CellCoverage {
    pub(crate) modified_resolutions: [bool; 16],

    /// cells by their resolution. The index of the array is the resolution for the referenced vec
    pub(crate) cells_by_resolution: [Vec<CellIndex>; 16],
}

impl CellCoverage {
    pub fn append(&mut self, other: &mut Self) {
        for ((r_idx, sink), source) in self
            .cells_by_resolution
            .iter_mut()
            .enumerate()
            .zip(other.cells_by_resolution.iter_mut())
        {
            if source.is_empty() {
                continue;
            }
            self.modified_resolutions[r_idx] = true;
            sink.append(source);
        }
    }

    /// check if the coverage covers the given cell.
    ///
    /// This method is far from efficient and should only be used sparingly.
    pub fn covers(&self, cell: CellIndex) -> bool {
        let cell_res = cell.resolution();
        for res in Resolution::range(Resolution::Zero, cell_res) {
            let search_cell = if res == cell_res {
                cell
            } else {
                cell.parent(res).unwrap()
            };
            if self.cells_by_resolution[u8::from(res) as usize].contains(&search_cell) {
                return true;
            }
        }
        false
    }

    pub fn compact(&mut self) -> Result<(), Error> {
        self.dedup(false, false);

        if let Some((min_touched_res, _)) = self
            .modified_resolutions
            .iter()
            .enumerate()
            .rev()
            .find(|(_, modified)| **modified)
        {
            let mut res = Some(Resolution::try_from(min_touched_res as u8)?);

            while let Some(h3_res) = res {
                let r_idx: usize = h3_res.into();
                let mut compacted_in = std::mem::take(&mut self.cells_by_resolution[r_idx]);
                compacted_in.sort();
                compacted_in.dedup();
                for cell in CellIndex::compact(compacted_in.into_iter())? {
                    self.insert(cell);
                }
                res = h3_res.pred();
            }

            // mark all resolutions as not-modified
            self.modified_resolutions
                .iter_mut()
                .for_each(|r| *r = false);
        }

        self.dedup(true, true);

        Ok(())
    }

    pub fn compacted_iter(&self) -> Box<dyn Iterator<Item = CellIndex> + '_> {
        Box::new(
            self.cells_by_resolution
                .iter()
                .flat_map(|v| v.iter())
                .copied(),
        )
    }

    pub fn into_compacted_iter(self) -> Box<dyn Iterator<Item = CellIndex>> {
        Box::new(
            self.cells_by_resolution
                .into_iter()
                .flat_map(|v| v.into_iter()),
        )
    }

    pub fn uncompacted_iter(&self, r: Resolution) -> Box<dyn Iterator<Item = CellIndex> + '_> {
        let r_idx: usize = r.into();
        Box::new((0..=r_idx).flat_map(move |r_idx| {
            self.cells_by_resolution[r_idx]
                .iter()
                .flat_map(move |cell| cell.children(r))
        }))
    }

    pub fn into_uncompacted_iter(mut self, r: Resolution) -> Box<dyn Iterator<Item = CellIndex>> {
        let r_idx: usize = r.into();
        Box::new((0..=r_idx).flat_map(move |r_idx| {
            std::mem::take::<Vec<CellIndex>>(&mut self.cells_by_resolution[r_idx])
                .into_iter()
                .flat_map(move |cell| cell.children(r))
        }))
    }

    pub fn len(&self) -> usize {
        self.cells_by_resolution.iter().map(|v| v.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        !self.cells_by_resolution.iter().any(|v| !v.is_empty())
    }

    pub fn insert(&mut self, cell: CellIndex) {
        let idx: usize = cell.resolution().into();
        self.cells_by_resolution[idx].push(cell);
        self.modified_resolutions[idx] = true;
    }

    pub fn dedup(&mut self, shrink: bool, parents: bool) {
        self.cells_by_resolution.par_iter_mut().for_each(|v| {
            v.sort();
            v.dedup();
            if shrink {
                v.shrink_to_fit();
            }
        });

        if parents
            && self
                .cells_by_resolution
                .iter()
                .filter(|v| !v.is_empty())
                .count()
                > 1
        {
            // remove cells whose parents are already contained
            let mut seen = HashSet::default();
            for v in self.cells_by_resolution.iter_mut() {
                if !seen.is_empty() {
                    v.retain(|cell| {
                        let mut is_contained = false;
                        let mut r = Some(cell.resolution());
                        while let Some(resolution) = r {
                            if let Some(cell) = cell.parent(resolution) {
                                if seen.contains(&cell) {
                                    is_contained = true;
                                    break;
                                }
                            }
                            r = resolution.pred();
                        }
                        !is_contained
                    });
                }
                seen.extend(v.iter().copied());
            }
        }
    }

    pub fn finalize(&mut self, compact: bool) -> Result<(), Error> {
        if compact {
            self.compact()?;
        } else {
            self.dedup(true, true);
        }
        Ok(())
    }
}

#[allow(clippy::derivable_impls)]
impl Default for CellCoverage {
    fn default() -> Self {
        Self {
            modified_resolutions: [false; 16],
            cells_by_resolution: Default::default(),
        }
    }
}
