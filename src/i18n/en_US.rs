use crate::i18n::{Language};

pub struct Language_zh_CN {}

impl Language for Language_zh_CN {
    const INFO: &'static str = "Info";
    const SUCCESS: &'static str = "Success";
    const WARNING: &'static str = "Warning";
    const ERROR: &'static str = "Error";
}
