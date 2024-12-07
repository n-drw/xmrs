use crate::prelude::*;
use bincode::error::DecodeError;

impl Module {
    /// Try to import Amiga Module file
    #[cfg(feature = "import_amiga")]
    pub fn load_mod(source: &[u8]) -> Result<Self, DecodeError> {
        use super::amiga::amiga_module::AmigaModule;

        match AmigaModule::load(source) {
            Ok(m) => Ok(m.to_module()),
            Err(e) => Err(e),
        }
    }

    /// Try to import Fast Tracker II Module file
    #[cfg(feature = "import_xm")]
    pub fn load_xm(source: &[u8]) -> Result<Self, DecodeError> {
        use super::xm::xmmodule::XmModule;

        match XmModule::load(source) {
            Ok(m) => Ok(m.to_module()),
            Err(e) => Err(e),
        }
    }

    /// Try to import Scream Tracker 3 Module file
    #[cfg(feature = "import_s3m")]
    pub fn load_s3m(source: &[u8]) -> Result<Self, DecodeError> {
        use super::s3m::s3m_module::S3mModule;

        match S3mModule::load(source) {
            Ok(m) => Ok(m.to_module()),
            Err(e) => Err(e),
        }
    }

    /// Try to import Impulse Tracker Module file
    #[cfg(feature = "import_it")]
    pub fn load_it(source: &[u8]) -> Result<Self, DecodeError> {
        use super::it::it_module::ItModule;

        match ItModule::load(source) {
            Ok(m) => Ok(m.to_module()),
            Err(e) => Err(e),
        }
    }

    /// Try to import any historical Module file
    pub fn load(source: &[u8]) -> Result<Self, DecodeError> {
        #[cfg(feature = "import_xm")]
        if let Ok(m) = Self::load_xm(source) {
            return Ok(m);
        };

        #[cfg(feature = "import_s3m")]
        if let Ok(m) = Self::load_s3m(source) {
            return Ok(m);
        };

        #[cfg(feature = "import_it")]
        if let Ok(m) = Self::load_it(source) {
            return Ok(m);
        };

        // The amiga format is the last one because it is the least well specified for format detection
        #[cfg(feature = "import_amiga")]
        if let Ok(m) = Self::load_mod(source) {
            return Ok(m);
        };

        return Err(DecodeError::Other("Unknown data?"));
    }
}
