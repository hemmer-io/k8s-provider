//! Resource modules

pub mod authorization;
pub use authorization::Authorization;
pub mod selfsubjectaccessreview;
pub use selfsubjectaccessreview::Selfsubjectaccessreview;
pub mod selfsubjectrulesreview;
pub use selfsubjectrulesreview::Selfsubjectrulesreview;
pub mod subjectaccessreview;
pub use subjectaccessreview::Subjectaccessreview;
pub mod localsubjectaccessreview;
pub use localsubjectaccessreview::Localsubjectaccessreview;
pub mod v1;
pub use v1::V1;

