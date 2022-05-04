use super::{TranslationError, Translator};
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
}

impl TranslatorBaidu {
    pub fn new(app_id: &str, api_key: &str) -> TranslatorBaidu {
        TranslatorBaidu {
            app_id: Box::new(app_id.to_string()),
            api_key: Box::new(api_key.to_string()),
        }
    }
}

impl Translator for TranslatorBaidu {
    /// Translate `text` from one language to another with Baidu Translate API.
    ///
    /// Reference: [通用翻译API接入文档](https://api.fanyi.baidu.com/doc/21)
    ///
    /// # Example
    /// ```rust
    /// let translator = TranslatorBaidu::new("[YOUR_APP_ID]", "[YOUR_API_KEY]");
    ///
    /// println!("{}", translator.translate("Hello, world!", "en", "zh").unwrap());
    /// ```
    fn translate(
        &self,
        text: &str,
        from_lang: &str,
        to_lang: &str,
    ) -> Result<String, TranslationError> {
        // Create salt for randomness
        let salt = rand::random::<[char; 4]>().iter().collect::<String>();
        // Calculate query signature
        let query = format!("{}{}{}{}", self.app_id, text, salt, self.api_key);
        let signature = format!("{:x}", md5::compute(query.as_bytes()));
        // Generate request body
        let form = [
            ("q", text),
            ("from", from_lang),
            ("to", to_lang),
            ("appid", &self.app_id),
            ("salt", &salt),
            ("sign", &signature),
        ];

        // Send request and decode result
        let result = reqwest::blocking::Client::new()
            .post("https://fanyi-api.baidu.com/api/trans/vip/translate")
            .form(&form)
            .send();
        if result.is_err() {
            return Err(TranslationError {
                message: format!("NETWORK ERR: {}", result.err().unwrap()),
            });
        }
        let result_json = result.unwrap().json::<ResultBaidu>().unwrap();

        let error_code = &result_json.error_code;
        if error_code != "" && error_code.parse::<i32>().unwrap() != 0 {
            return Err(TranslationError {
                message: format!(
                    "API ERR: {} {}",
                    result_json.error_code, result_json.error_msg
                ),
            });
        }

        Ok(result_json.trans_result[0].dst.clone())
    }
}
