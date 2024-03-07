use serde::Serialize;

use crate::v1::generic::define_types;

define_types! {
    /// Type of project
    #[derive(Serialize, Copy, Clone, PartialEq, Eq)]
    pub enum ProjectTy: i64 {
        Legacy = 0,
    }
}
