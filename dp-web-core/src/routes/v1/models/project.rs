use serde::Serialize;

use crate::define_types;

define_types! {
    /// Type of user token
    #[derive(Serialize, Copy, Clone, PartialEq, Eq)]
    pub enum ProjectTy: i64 {
        Legacy = 0,
    }
}
