//! Resource modules

pub mod validatingadmissionpolicybinding;
pub use validatingadmissionpolicybinding::Validatingadmissionpolicybinding;
pub mod v1;
pub use v1::V1;
pub mod statu;
pub use statu::Statu;
pub mod validatingadmissionpolicie;
pub use validatingadmissionpolicie::Validatingadmissionpolicie;
pub mod mutatingwebhookconfiguration;
pub use mutatingwebhookconfiguration::Mutatingwebhookconfiguration;
pub mod validatingwebhookconfiguration;
pub use validatingwebhookconfiguration::Validatingwebhookconfiguration;

