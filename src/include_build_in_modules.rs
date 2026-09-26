use crate::semantic::{CodeModuleMetaData, CodeModuleSourceFileType};

pub fn builtin_modules() -> Vec<CodeModuleMetaData> {
    include!(concat!(env!("OUT_DIR"), "/builtin_modules.rs"))
}
