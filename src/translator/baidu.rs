use super::{LanguagePair, TranslationError, Translator};
use serde::Deserialize;

#[derive(Deserialize)]
struct ResultBaiduNode {
    #[serde(default)]
    pub dst: String,
}

#[derive(Deserialize)]
struct ResultBaidu {
    #[serde(default = "Vec::new")]
    pub trans_result: Vec<ResultBaiduNode>,

    #[serde(default)]
    pub error_code: String,

    #[serde(default)]
    pub error_msg: String,
}

pub struct TranslatorBaidu {
    pub app_id: Box<String>,
    pub api_key: Box<String>,
    client: reqwest::blocking::Client,
}

impl TranslatorBaidu {
    pub fn new(app_id: &str, api_key: &str) -> TranslatorBaidu {
        TranslatorBaidu {
            app_id: Box::new(app_id.to_string()),
            api_key: Box::new(api_key.to_string()),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Translator for TranslatorBaidu {
    /// Translate `text` from one language to another with Baidu Translation API.
    ///
    /// Reference: [通用翻译API接入文档](https://api.fanyi.baidu.com/doc/21)
    ///
    /// # Example
    /// ```rust
    /// let translator = TranslatorBaidu::new("[YOUR_APP_ID]", "[YOUR_API_KEY]");
    /// let lang = LanguagePair { from_lang: "en".to_string(), to_lang: "zh".to_string() };
    ///
    /// println!("{}", translator.translate("Hello, world!", &lang).unwrap());
    /// ```
    fn translate(&self, text: &str, lang: &LanguagePair) -> Result<String, TranslationError> {
        // Create salt for randomness
        let salt = rand::random::<[char; 4]>().iter().collect::<String>();
        // Calculate query signature
        let query = format!("{}{}{}{}", self.app_id, text, salt, self.api_key);
        let signature = format!("{:x}", md5::compute(query.as_bytes()));
        // Generate request body
        let form = [
            ("q", text),
            ("from", &lang.from_lang),
            ("to", &lang.to_lang),
            ("appid", &self.app_id),
            ("salt", &salt),
            ("sign", &signature),
        ];

        // Send request
        let result = self
            .client
            .post("https://fanyi-api.baidu.com/api/trans/vip/translate")
            .form(&form)
            .send();

        // Handle network error
        if let Err(e) = result {
            return Err(TranslationError::new(
                format!("NETWORK ERR: {}", e).as_str(),
            ));
        }
        let result_json = result.unwrap().json::<ResultBaidu>().unwrap();
        // Handle API error
        let error_code = &result_json.error_code;
        if error_code != "" && error_code.parse::<i32>().unwrap() != 0 {
            return Err(TranslationError::new(
                format!(
                    "API ERR: {} {}",
                    result_json.error_code, result_json.error_msg
                )
                .as_str(),
            ));
        }

        Ok(result_json.trans_result[0].dst.clone())
    }
}
