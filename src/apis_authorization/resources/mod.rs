//! Resource modules

pub mod localsubjectaccessreview;
pub use localsubjectaccessreview::Localsubjectaccessreview;
pub mod selfsubjectaccessreview;
pub use selfsubjectaccessreview::Selfsubjectaccessreview;
pub mod selfsubjectrulesreview;
pub use selfsubjectrulesreview::Selfsubjectrulesreview;
pub mod subjectaccessreview;
pub use subjectaccessreview::Subjectaccessreview;
pub mod v1;
pub use v1::V1;

