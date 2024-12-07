#![forbid(unsafe_code)]

pub(crate) mod import_memory;
pub(crate) mod orders_helper;
pub(crate) mod patternslot;
pub(crate) mod track_import_effect;
pub(crate) mod track_import_unit;

/// impl loader to Module. See Module documentation load* fn.
mod import_loader;

/// Load and Save Historical XM files
#[cfg(feature = "import_xm")]
pub mod xm;

/// Load only Historical Amiga MOD files
#[cfg(feature = "import_amiga")]
pub mod amiga;

/// Load only Historical IT files
#[cfg(feature = "import_it")]
pub mod it;

/// Load only Historical S3M files
#[cfg(feature = "import_s3m")]
pub mod s3m;

/// Load only Historical SID files
#[cfg(feature = "import_sid")]
pub mod sid;
