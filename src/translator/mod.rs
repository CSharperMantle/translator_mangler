pub mod baidu;
pub mod google;
pub mod youdao;

/// Errors that may occur when translating.
#[derive(Debug)]
pub struct TranslationError {
    /// The message string describing the error.
    pub message: String,
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
    /// let text = translator.translate("Hello, world!", &lang).unwrap();
    /// ```
    fn translate(&self, text: &str, lang: &LanguagePair) -> Result<String, TranslationError>;

    /// Get a list of all supported languages.
    /// 
    /// # Example
    /// ```rust
    /// let translator: Translator = ...;
    /// 
    /// let languages = translator.get_languages();
    /// ```
    fn get_supported_langs(&self) -> &[&'static str];

    /// Check if a language is supported.
    /// 
    /// # Arguments
    /// * `single_lang` - The language to check.
    /// 
    /// # Example
    /// ```rust
    /// let translator: Translator = ...;
    /// 
    /// let is_supported = translator.is_single_lang_supported("klingon");
    /// ```
    fn is_single_lang_supported(&self, single_lang: &str) -> bool;
}

/// A pair of languages to translate between.
pub struct LanguagePair {
    pub from_lang: String,
    pub to_lang: String,
}
