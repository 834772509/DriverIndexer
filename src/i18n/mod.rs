use crate::i18n::en_US::Language_zh_CN;

pub mod en_US;
pub mod zh_CN;

pub trait Language {
    const INFO: &'static str;
    const SUCCESS: &'static str;
    const WARNING: &'static str;
    const ERROR: &'static str;
    fn new() -> Language_zh_CN {
        Language_zh_CN {}
    }
    fn getContent(constant: &str) {
        println!("{}", constant);
    }
}
