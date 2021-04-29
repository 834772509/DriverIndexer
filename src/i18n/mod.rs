use std::collections::HashMap;
use crate::bindings::Windows;
use unic_langid::{LanguageIdentifier, langid};
use fluent_templates::{Loader, static_loader};
use fluent_templates::fluent_bundle::FluentValue;

// 多国语言支持
const US_ENGLISH: LanguageIdentifier = langid!("en-US");
const ZH_CHINEXE: LanguageIdentifier = langid!("zh-CN");

static_loader! {
    pub static LOCALES = {
        locales: "./src/i18n",
        core_locales: "./src/i18n/core.ftl",
        fallback_language: "en-US",
        customise: |bundle| bundle.set_use_isolating(false),
    };
}

pub fn getLocaleText(id: &str, args: Option<&HashMap<String, FluentValue>>) -> String {
    lazy_static! {
        pub static ref LANG_ID: u16 = unsafe { return Windows::Win32::Intl::GetUserDefaultUILanguage() };
    }
    let lang = if LANG_ID.eq(&2052) { ZH_CHINEXE } else { US_ENGLISH };
    if !args.is_some() {
        LOCALES.lookup(&lang, id)
    } else {
        LOCALES.lookup_with_args(&lang, id, &args.unwrap())
    }
}
