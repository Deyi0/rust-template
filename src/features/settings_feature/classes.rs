// IMPORT //

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

// CLASS //

pub trait Settings: Debug + Clone + Default + Serialize + for<'a> Deserialize<'a> {
    fn get_version(&self) -> u16;
}