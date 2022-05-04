use super::translator::{TranslationError, Translator};
use rand::seq::SliceRandom;

/// Mangle the `original_text` by translating it for many times with `translator`.
///
/// # Arguments
/// * `translator` - The translator to use for mangling.
/// * `original_text` - The text to mangle.
/// * `original_lang` - The language of `original_text`.
/// * `langs` - The list of languages to use during mangling.
/// * `rounds` - The number of rounds to translate.
/// * `delay` - The delay in milliseconds between each translation request.
///
/// # Returns
/// The mangled text.
///
/// # Example
/// ```rust
/// let result = mangle(&translator, "Ignorance is strength.", "en", &["en", "zh"], 20, 1000);
/// ```
pub fn mangle(
    translator: &Box<dyn Translator>,
    original_text: &str,
    original_lang: &str,
    langs: &[&str],
    rounds: u32,
    delay: u64,
) -> Result<String, TranslationError> {
    let mut mangled_text = original_text.to_string();
    let mut rng = rand::thread_rng();

    // Transform the text one language by another
    let last_lang: Result<&str, TranslationError> = (0..rounds)
        .map(|_| {
            langs.choose(&mut rng).unwrap()
        })
        .fold(Ok(original_lang), |prev_lang, current_lang| {
            mangled_text = translator.translate(&mangled_text, prev_lang?, current_lang)?;
            // Sleep a while to prevent API abuse
            std::thread::sleep(std::time::Duration::from_millis(delay));

            Ok(current_lang)
        });

    // Translate the text back to the original language
    Ok(translator.translate(&mangled_text, last_lang?, original_lang)?)
}
