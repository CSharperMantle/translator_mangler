use super::{LanguagePair, TranslationError, Translator};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateTextResponseTranslation {
    #[serde(default)]
    pub translated_text: String,
}

#[derive(Deserialize)]
struct TranslateTextResponseList {
    #[serde(default = "Vec::new")]
    pub translations: Vec<TranslateTextResponseTranslation>,
}

#[derive(Deserialize)]
struct ResultGoogleCloud {
    pub data: TranslateTextResponseList,
}

pub struct TranslatorGoogleCloud {
    pub api_key: Box<String>,
    client: reqwest::blocking::Client,
}

impl TranslatorGoogleCloud {
    pub fn new(api_key: &str) -> TranslatorGoogleCloud {
        TranslatorGoogleCloud {
            api_key: Box::new(api_key.to_string()),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Translator for TranslatorGoogleCloud {
    /// Translate `text` from one language to another with Google Cloud Translation API.
    ///
    /// Reference: [Google Cloud Docs](https://cloud.google.com/translate/docs/reference/rest/v2/translate)
    ///
    /// # Example
    /// ```rust
    /// let translator = TranslatorGoogleCloud::new("[YOUR_API_KEY]");
    /// let lang = LanguagePair { from_lang: "en".to_string(), to_lang: "zh".to_string() };
    ///
    /// println!("{}", translator.translate("Hello, world!", &lang).unwrap());
    /// ```
    fn translate(&self, text: &str, lang: &LanguagePair) -> Result<String, TranslationError> {
        // Generate request body
        let form = [
            ("q", text),
            ("source", &lang.from_lang),
            ("target", &lang.to_lang),
            ("key", &self.api_key),
            ("format", "text"),
            ("model", "base"),
        ];

        // Send request
        let result = self
            .client
            .post("https://translation.googleapis.com/language/translate/v2")
            .form(&form)
            .send();
        // Handle network error
        if let Err(e) = result {
            return Err(TranslationError::new(
                format!("NETWORK ERR: {}", e).as_str(),
            ));
        }
        let unwrapped_result = result.unwrap();

        let status = unwrapped_result.status();
        if !status.is_success() {
            return Err(TranslationError::new(
                format!("REQUEST ERR: HTTP {}", status).as_str(),
            ));
        }

        let result_json = unwrapped_result.json::<ResultGoogleCloud>().unwrap();
        // Handle API error
        Ok(result_json.data.translations[0].translated_text.clone())
    }
}
