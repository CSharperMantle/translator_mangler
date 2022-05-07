use super::{LanguagePair, TranslationError, Translator};

use sha2::Digest;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResultYoudao {
    #[serde(default)]
    pub error_code: String,

    #[serde(default)]
    pub translation: Vec<String>,
}

/// A translator using Youdao AI as its backend.
///
/// Reference: [有道智云AI开放平台](https://ai.youdao.com/)
pub struct TranslatorYoudao {
    pub app_key: Box<String>,
    pub app_secret: Box<String>,
    client: reqwest::blocking::Client,
}

impl TranslatorYoudao {
    /// Create an instance of `TranslatorYoudao` with the given API key and App ID.
    ///
    /// # Arguments
    /// * `app_key` - Your App ID.
    /// * `app_secret` - Your App secret.
    ///
    /// # Returns
    /// A new instance of `TranslatorYoudao`.
    ///
    /// # Example
    /// ```rust
    /// let translator = TranslatorYoudao::new("[YOUR_APP_KEY]", "[YOUR_APP_SECRET]");
    /// ```
    pub fn new(app_key: &str, app_secret: &str) -> TranslatorYoudao {
        TranslatorYoudao {
            app_key: Box::new(app_key.to_string()),
            app_secret: Box::new(app_secret.to_string()),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Translator for TranslatorYoudao {
    /// Translate `text` from one language to another with Youdao AI.
    ///
    /// Reference: [有道智云AI开放平台](https://ai.youdao.com/)
    ///
    /// # Example
    /// ```rust
    /// let translator = TranslatorYoudao::new("[YOUR_APP_KEY]", "[YOUR_APP_SECRET]");
    /// let lang = LanguagePair { from_lang: "en".to_string(), to_lang: "zh".to_string() };
    ///
    /// println!("{}", translator.translate("Hello, world!", &lang).unwrap());
    /// ```
    fn translate(&self, text: &str, lang: &LanguagePair) -> Result<String, TranslationError> {
        // Get current UNIX timestamp
        let time_utc_unix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // Create truncated input
        let input = if text.len() > 20 {
            text[0..10].to_string() + &text.len().to_string() + &text[text.len() - 10..]
        } else {
            text.to_string()
        };
        // Create salt for randomness
        let salt = rand::random::<[char; 4]>().iter().collect::<String>();
        // Create signature
        let signature = hex::encode(
            sha2::Sha256::new()
                .chain_update(
                    format!(
                        "{}{}{}{}{}",
                        self.app_key, input, salt, time_utc_unix, self.app_secret
                    )
                    .as_bytes(),
                )
                .finalize(),
        );
        // Generate request body
        let form = [
            ("q", text),
            ("from", &lang.from_lang),
            ("to", &lang.to_lang),
            ("appKey", &self.app_key),
            ("salt", &salt),
            ("sign", &signature),
            ("signType", "v3"),
            ("curtime", &time_utc_unix.to_string()),
        ];

        // Send request
        let result = self
            .client
            .post("https://openapi.youdao.com/api")
            .form(&form)
            .send();

        // Handle network error
        if let Err(e) = result {
            return Err(TranslationError::new(
                format!("NETWORK ERR: {}", e).as_str(),
            ));
        }
        let result_json = result.unwrap().json::<ResultYoudao>().unwrap();
        // Handle API error
        let error_code = &result_json.error_code;
        if error_code != "" && error_code.parse::<i32>().unwrap() != 0 {
            return Err(TranslationError::new(&format!(
                "API ERR: {}",
                result_json.error_code
            )));
        }

        Ok(result_json.translation[0].clone())
    }
}