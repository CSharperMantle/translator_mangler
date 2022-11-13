use crate::translator::{TranslationDirection, TranslationError, Translator};
use rand::seq::SliceRandom;

/// Get a random 'path' of languages, starting from and ending with
/// `original_language`. The nodes in the way is randomly selected from
/// `lang_bank`, with a total of `rounds` nodes.
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
/// let pairs = get_random_lang_path(original_lang, &lang_bank, rounds);
/// // A possible result:
/// // vec![
/// //    TranslationDirection { from_lang: "en".to_string(), to_lang: "fr".to_string() },
/// //    TranslationDirection { from_lang: "fr".to_string(), to_lang: "en".to_string() },
/// //    TranslationDirection { from_lang: "en".to_string(), to_lang: "fr".to_string() },
/// //    TranslationDirection { from_lang: "fr".to_string(), to_lang: "zh".to_string() },
/// //    TranslationDirection { from_lang: "zh".to_string(), to_lang: "en".to_string() },
/// // ]
/// ```
pub fn get_random_lang_path(
    original_lang: &str,
    lang_bank: &[&str],
    rounds: usize,
) -> Vec<TranslationDirection> {
    // Fast-fail
    if lang_bank.is_empty() {
        return vec![TranslationDirection {
            from_lang: original_lang.to_string(),
            to_lang: original_lang.to_string(),
        }];
    }

    let mut rng = rand::thread_rng();

    let mut prev_lang = original_lang.to_string();
    let mut langs: Vec<TranslationDirection> = Vec::with_capacity(rounds);
    for i in 0..rounds {
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
        langs.push(TranslationDirection {
            from_lang: prev_lang,
            to_lang: next_lang.clone(),
        });
        // Move on.
        prev_lang = next_lang;
    }

    langs
}

/// Mangle the `original_text` by translating it for many times with `translator`.
///
/// # Arguments
/// * `translator` - The translator to use for mangling.
/// * `original_text` - The text to mangle.
/// * `lang_path` - The 'path' of languages the text should be mangled in.
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
    translator: &dyn Translator,
    original_text: &str,
    lang_path: &[TranslationDirection],
    delay: u64,
) -> Result<String, TranslationError> {
    //Reduce the langs list, apply translation to each pair.
    lang_path
        .iter()
        .fold(Ok(original_text.to_string()), |acc, current| match acc {
            Ok(last) => {
                std::thread::sleep(std::time::Duration::from_millis(delay));
                translator.translate(&last, current)
            }
            Err(e) => Err(e),
        })
}
