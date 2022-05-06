use super::translator::{LanguagePair, TranslationError, Translator};
use rand::seq::SliceRandom;

/// Get a random list of `LanguagePair`s with `to_lang` selected from `lang_bank` randomly.
///
/// # Arguments
/// * `original_lang` - The original language to translate to.
/// * `lang_bank` - The list of languages to choose from.
/// * `rounds` - The number of pairs to get.
///
/// # Example
/// ```rust
/// let lang_bank = vec![
///    "en", "zh", "fr"
/// ];
/// let original_lang = "en";
/// let rounds = 5;
///
/// let pairs = get_random_lang_pairs(original_lang, &lang_bank, rounds);
/// // A possible result:
/// // vec![
/// //    LanguagePair { from_lang: "en".to_string(), to_lang: "fr".to_string() },
/// //    LanguagePair { from_lang: "fr".to_string(), to_lang: "en".to_string() },
/// //    LanguagePair { from_lang: "en".to_string(), to_lang: "fr".to_string() },
/// //    LanguagePair { from_lang: "fr".to_string(), to_lang: "zh".to_string() },
/// //    LanguagePair { from_lang: "zh".to_string(), to_lang: "en".to_string() },
/// // ]
/// ```
pub fn get_random_lang_pairs(
    original_lang: &str,
    lang_bank: &[&str],
    rounds: usize,
) -> Vec<LanguagePair> {
    // Fast-fail
    if lang_bank.len() < 1 {
        return vec![LanguagePair {
            from_lang: original_lang.to_string(),
            to_lang: original_lang.to_string(),
        }];
    }

    let mut rng = rand::thread_rng();

    let mut prev_lang = original_lang.to_string();
    let langs: Vec<LanguagePair> = (0..rounds)
        .map(|i| {
            // Choose a random language from the lang_bank,
            // or original_lang if we have reached the last round.
            let next_lang = if i == (rounds - 1) {
                original_lang.to_string()
            } else {
                let mut l = lang_bank.choose(&mut rng).unwrap();
                // Make sure we don't get the same language twice
                // to avoid request waste.
                while l == &prev_lang {
                    l = lang_bank.choose(&mut rng).unwrap();
                }
                l.to_string()
            };
            let pair = LanguagePair {
                from_lang: prev_lang.clone(),
                to_lang: next_lang.to_string(),
            };
            // Move on.
            prev_lang = next_lang.to_string();
            pair
        })
        .collect();

    langs
}

/// Mangle the `original_text` by translating it for many times with `translator`.
///
/// # Arguments
/// * `translator` - The translator to use for mangling.
/// * `original_text` - The text to mangle.
/// * `langs` - The list of language pairs to use during mangling.
/// * `delay` - The delay in milliseconds between each translation request.
///
/// # Returns
/// The mangled text.
///
/// # Example
/// ```rust
/// let langs = get_random_lang_pairs("en", &["en", "zh"], 20);
/// let result = mangle(&translator, "Ignorance is strength.", &langs, 1000);
/// ```
pub fn mangle(
    translator: &Box<dyn Translator>,
    original_text: &str,
    langs: &[LanguagePair],
    delay: u64,
) -> Result<String, TranslationError> {
    // Traverse the langs to apply translation according to each pair,
    // and then reduce the result to get the final translation.
    langs
        .iter()
        .fold(Ok(original_text.to_string()), |acc, lang_pair| match acc {
            Ok(last_result) => {
                std::thread::sleep(std::time::Duration::from_millis(delay));
                translator.translate(&last_result, lang_pair)
            }
            Err(e) => Err(e),
        })
}
