pub mod baidu;

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
    ///
    /// translator.translate("Hello, world!", "en", "zh").unwrap();
    /// ```
    fn translate(
        &self,
        text: &str,
        from_lang: &str,
        to_lang: &str,
    ) -> Result<String, TranslationError>;
}
