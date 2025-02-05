use std::{collections::VecDeque, marker::PhantomData};

use anyhow::Result;

pub struct StateManager<Args: Copy> {
    args: PhantomData<Args>,
    funcs: VecDeque<Box<dyn FnOnce(Args) -> Result<()>>>,
}
impl<Args: Copy> Default for StateManager<Args> {
    fn default() -> Self {
        Self {
            args: Default::default(),
            funcs: Default::default(),
        }
    }
}
impl<Args: Copy> StateManager<Args> {
    pub const fn new() -> Self {
        Self {
            args: PhantomData,
            funcs: VecDeque::new(),
        }
    }

    pub fn add(&mut self, func: Box<dyn FnOnce(Args) -> Result<()>>) {
        self.funcs.push_back(func);
    }

    pub fn apply(&mut self, args: Args) -> Result<bool> {
        let mut edited = false;
        while let Some(func) = self.funcs.pop_front() {
            edited = true;
            func(args)?;
        }
        Ok(edited)
    }
}
