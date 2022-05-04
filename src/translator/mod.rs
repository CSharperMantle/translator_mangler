pub mod baidu;

#[derive(Debug)]
pub struct TranslationError {
    pub message: String,
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
