use paste::paste;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

/// Input and compute cell definition.
///
/// Having input and compute cells be variants of an enumeration provides an easier interface
/// to work with, but does add a lot more matching.
enum Cell<'a, T: Copy> {
    Input {
        value: T,
        dependents: Vec<ComputeCellID>,
    },
    Compute {
        value: T,
        dependents: Vec<ComputeCellID>,

        dirty: bool,
        dependencies: Vec<CellID>,
        compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
        callbacks_i: std::ops::RangeFrom<usize>,
        callbacks: HashMap<CallbackID, Box<dyn FnMut(T) -> () + 'a>>,
    },
}

macro_rules! cell_getters {
    (@variants both, $cell:tt, $field:tt) => {
        match $cell {
            Cell::Input { $field, .. } => $field,
            Cell::Compute { $field, .. } => $field,
        }
    };

    (@variants compute, $cell:tt, $field:tt) => {
        match $cell {
            Cell::Compute { $field, .. } => $field,
            _ => panic!(),
        }
    };

    (ref, $variants:tt, $field:tt, $type:ty) => {
        pub fn $field(&self) -> &$type {
            cell_getters!(@variants $variants, self, $field)
        }
    };

    (mut, $variants:tt, $field:tt, $type:ty) => {
        paste! {
            pub fn [<$field _mut>](&mut self) -> &mut $type {
                cell_getters!(@variants $variants, self, $field)
            }
        }
    };

    (both, $variants:tt, $field:tt, $type:ty) => {
        cell_getters!(ref, $variants, $field, $type);
        cell_getters!(mut, $variants, $field, $type);
    };
}

impl<'a, T: Copy> Cell<'a, T> {
    /// New input cell variant.
    pub fn new_input(initial: T) -> Self {
        Self::Input {
            value: initial,
            dependents: Vec::new(),
        }
    }

    /// New compute cell variant.
    pub fn new_compute<F>(initial: T, dependencies: Vec<CellID>, f: F) -> Self
    where
        F: Fn(&[T]) -> T + 'a,
    {
        Self::Compute {
            value: initial,
            dependents: Vec::new(),

            dirty: false,
            dependencies,
            compute_func: Box::new(f),
            callbacks_i: 0..,
            callbacks: HashMap::new(),
        }
    }

    /// Is cell marked clean. (Input cells are always clean.)
    pub fn is_clean(&self) -> bool {
        match self {
            Cell::Compute { dirty, .. } => !*dirty,
            _ => true,
        }
    }

    /// Mark cell as dirty. (Compute cells only!)
    pub fn mark_dirty(&mut self) {
        match self {
            Cell::Compute { dirty, .. } => *dirty = true,
            _ => panic!(),
        }
    }

    /// Mark cell as clean. (Compute cells only!)
    pub fn mark_clean(&mut self) {
        match self {
            Cell::Compute { dirty, .. } => *dirty = false,
            _ => panic!(),
        }
    }

    /// Insert a callback and get the ID. (Compute cells only!)
    pub fn insert_callback<F>(&mut self, f: F) -> CallbackID
    where
        F: FnMut(T) -> () + 'a,
    {
        match self {
            Cell::Compute {
                callbacks_i,
                callbacks,
                ..
            } => {
                let new_id = CallbackID(callbacks_i.next().unwrap());
                callbacks.insert(new_id, Box::new(f));
                new_id
            }
            _ => panic!(),
        }
    }

    cell_getters!(both, both, value, T);
    cell_getters!(both, both, dependents, Vec<ComputeCellID>);
    cell_getters!(ref, compute, dependencies, Vec<CellID>);
    cell_getters!(ref, compute, compute_func, Box<dyn Fn(&[T]) -> T + 'a>);
    cell_getters!(mut, compute, callbacks, HashMap<CallbackID, Box<dyn FnMut(T) -> () + 'a>>);
}

pub struct Reactor<'a, T: Copy> {
    input_cells: Vec<Cell<'a, T>>,
    compute_cells: Vec<Cell<'a, T>>,
}

impl<'a, T> Reactor<'a, T>
where
    T: Copy + PartialEq + Debug,
{
    // Constructs a new reactor.
    pub fn new() -> Self {
        Self {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    /// Constructs a new input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let new_id = InputCellID(self.input_cells.len());
        self.input_cells.push(Cell::new_input(initial));
        new_id
    }

    /// Constructs a new compute cell with the specified dependencies and compute function,
    /// returning its ID.
    pub fn create_compute<F>(
        &mut self,
        dependencies: &[CellID],
        f: F,
    ) -> Result<ComputeCellID, CellID>
    where
        F: Fn(&[T]) -> T + 'a,
    {
        if let Some(&id) = dependencies.iter().find(|&id| self.cell(id).is_none()) {
            Err(id)
        } else {
            let new_id = ComputeCellID(self.compute_cells.len());
            for id in dependencies.iter() {
                self.cell_mut(id).unwrap().dependents_mut().push(new_id);
            }

            let initial = f(&self.values(dependencies));
            let dependencies = dependencies.to_vec();
            let f = Box::new(f);

            let new_cell = Cell::new_compute(initial, dependencies, f);
            self.compute_cells.push(new_cell);
            Ok(new_id)
        }
    }

    /// Provides a reference to the cell with the given cell ID.
    fn cell(&self, id: &CellID) -> Option<&Cell<'a, T>> {
        match id {
            &CellID::Input(InputCellID(id)) => self.input_cells.get(id),
            &CellID::Compute(ComputeCellID(id)) => self.compute_cells.get(id),
        }
    }

    /// Provides a mutable reference to the cell with the given cell ID.
    fn cell_mut(&mut self, id: &CellID) -> Option<&mut Cell<'a, T>> {
        match id {
            &CellID::Input(InputCellID(id)) => self.input_cells.get_mut(id),
            &CellID::Compute(ComputeCellID(id)) => self.compute_cells.get_mut(id),
        }
    }

    /// Collects a vector of the values from the given cell IDs.
    fn values(&self, ids: &[CellID]) -> Vec<T> {
        ids.iter().map(|&id| self.value(id).unwrap()).collect()
    }

    /// Gets the value of the specified cell.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(id)) => self
                .input_cells
                .get(id)
                .and_then(|cell| Some(*cell.value())),
            CellID::Compute(ComputeCellID(id)) => self
                .compute_cells
                .get(id)
                .and_then(|cell| Some(*cell.value())),
        }
    }

    /// Sets the value of the specified input cell.
    ///
    /// Marks dirty all the compute cells in the dependency tree
    /// starting at the specified input cell.
    pub fn set_value(&mut self, InputCellID(id): InputCellID, new_value: T) -> bool {
        if self.input_cells.get_mut(id).is_none() {
            return false;
        }

        let dependents = {
            let cell = self.input_cells.get_mut(id).unwrap();
            *cell.value_mut() = new_value;
            cell.dependents().clone()
        };

        for &id in dependents.iter() {
            self.mark_tree_dirty(id);
        }

        for id in dependents {
            self.update_compute_cell(id);
        }

        true
    }

    /// Marks the compute cells in a dependency tree as dirty.
    fn mark_tree_dirty(&mut self, ComputeCellID(id): ComputeCellID) {
        let dependents = {
            let cell = self.compute_cells.get_mut(id).unwrap();
            cell.mark_dirty();
            cell.dependents().clone()
        };
        for id in dependents {
            self.mark_tree_dirty(id);
        }
    }

    /// Attempts to update the specified compute cell.
    ///
    /// A compute cell is only updated if all of its dependency cells are marked
    /// clean (input cells are always clean, compute cells need to first be updated
    /// themselves). Then if an update occurs, and only if the value of the cell actually
    /// changed, the cell's callbacks are fired and dependent cells are updated recursively.
    fn update_compute_cell(&mut self, ComputeCellID(id): ComputeCellID) {
        let all_clean = self
            .compute_cells
            .get(id)
            .unwrap()
            .dependencies()
            .iter()
            .map(|id| self.cell(id).unwrap())
            .all(|cell| cell.is_clean());

        if all_clean {
            let values = {
                let cell = self.compute_cells.get(id).unwrap();
                self.values(&cell.dependencies())
            };

            // Update cell value and get if it actually changed
            let value_changed = {
                let cell = self.compute_cells.get_mut(id).unwrap();
                let new_value = (cell.compute_func())(&values);
                let value_changed = &new_value != cell.value();

                *cell.value_mut() = new_value;
                cell.mark_clean();

                value_changed
            };

            // Fire callbacks and update dependents if value changed
            if value_changed {
                let dependents = {
                    let cell = self.compute_cells.get_mut(id).unwrap();
                    let value = *cell.value();

                    for (_, f) in cell.callbacks_mut().iter_mut() {
                        f(value);
                    }

                    cell.dependents().clone()
                };

                for id in dependents {
                    self.update_compute_cell(id);
                }
            }
        }
    }

    /// Adds a callback to the specified compute cell, returning its ID.
    pub fn add_callback<F: FnMut(T) -> () + 'a>(
        &mut self,
        ComputeCellID(id): ComputeCellID,
        f: F,
    ) -> Option<CallbackID> {
        self.compute_cells
            .get_mut(id)
            .map(|cell| cell.insert_callback(f))
    }

    /// Attempts to remove a callback from the specified compute cell.
    pub fn remove_callback(
        &mut self,
        ComputeCellID(id): ComputeCellID,
        callback_id: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cells
            .get_mut(id)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|cell| {
                cell.callbacks_mut()
                    .remove(&callback_id)
                    .and(Some(()))
                    .ok_or(RemoveCallbackError::NonexistentCallback)
            })
    }
}
