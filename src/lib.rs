mod mangler;
mod translator;

pub use mangler::{get_random_lang_path, mangle};
pub use translator::{
    baidu::TranslatorBaidu, google::TranslatorGoogleCloud, youdao::TranslatorYoudao, LanguagePair,
    TranslationError, Translator,
};
