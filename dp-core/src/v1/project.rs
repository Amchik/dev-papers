use serde::{Deserialize, Serialize};

use crate::v1::generic::define_types;

define_types! {
    /// Type of project
    #[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
    pub enum ProjectTy: i64 {
        Legacy = 0,
    }
}
