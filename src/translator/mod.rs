pub mod baidu;
pub mod google;

#[derive(Debug)]
pub struct TranslationError {
    pub message: String,
}

impl TranslationError {
    pub fn new(message: &str) -> TranslationError {
        TranslationError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TranslationError: {}", self.message)
    }
}

/// A trait to support text translation.
pub trait Translator {
    /// Translate `text` from one language to another by some means.
    ///
    /// # Arguments
    /// * `text` - The text to translate.
    /// * `from_lang` - The language of `text`.
    /// * `to_lang` - The language to translate `text` to.
    ///
    /// # Example
    /// ```rust
    /// let translator: Translator = ...;
    /// let lang = LanguagePair { from_lang: "en".to_string(), to_lang: "zh".to_string() };
    ///
    /// println!("{}", translator.translate("Hello, world!", &lang).unwrap());
    /// ```
    fn translate(&self, text: &str, lang: &LanguagePair) -> Result<String, TranslationError>;
}

/// A pair of languages to translate between.
pub struct LanguagePair {
    pub from_lang: String,
    pub to_lang: String,
}
