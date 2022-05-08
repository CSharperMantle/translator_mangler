mod mangler;
mod translator;

use core::panic;

use crate::mangler::{get_random_lang_path, mangle};
use crate::translator::{
    baidu::TranslatorBaidu, google::TranslatorGoogleCloud, youdao::TranslatorYoudao, Translator,
};

use dialoguer::{
    theme::{ColorfulTheme, Theme},
    Input, Select,
};

fn prompt_baidu_api(theme: &dyn Theme) -> Box<dyn Translator> {
    let input_api_key = Input::<String>::with_theme(theme)
        .with_prompt("API key for Baidu Translation API")
        .interact_text()
        .unwrap_or_default();
    let input_app_id = Input::<String>::with_theme(theme)
        .with_prompt("App ID for Baidu Translation API")
        .interact_text()
        .unwrap_or_default();

    Box::new(TranslatorBaidu::new(&input_app_id, &input_api_key))
}

fn prompt_google_cloud_api(theme: &dyn Theme) -> Box<dyn Translator> {
    let input_api_key = Input::<String>::with_theme(theme)
        .with_prompt("API key for Google Cloud Translation API")
        .interact_text()
        .unwrap_or_default();

    Box::new(TranslatorGoogleCloud::new(&input_api_key))
}

fn prompt_youdao_api(theme: &dyn Theme) -> Box<dyn Translator> {
    let input_app_key = Input::<String>::with_theme(theme)
        .with_prompt("App key for Youdao AI")
        .interact_text()
        .unwrap_or_default();
    let input_app_secret = Input::<String>::with_theme(theme)
        .with_prompt("App secret for Youdao AI")
        .interact_text()
        .unwrap_or_default();

    Box::new(TranslatorYoudao::new(&input_app_key, &&input_app_secret))
}

fn main() {
    let terminal_theme = ColorfulTheme::default();

    println!("[INFO] Welcome to translator_mangler!");

    let api_choices = ["Baidu", "Google Cloud", "Youdao AI"];
    let input_api_choices = Select::with_theme(&terminal_theme)
        .with_prompt("Back-end translation API")
        .default(0)
        .items(&api_choices)
        .interact()
        .unwrap_or_default();
    let arg_api_choice = api_choices[input_api_choices];
    let translator = match arg_api_choice {
        "Baidu" => prompt_baidu_api(&terminal_theme),
        "Google Cloud" => prompt_google_cloud_api(&terminal_theme),
        "Youdao AI" => prompt_youdao_api(&terminal_theme),
        _ => panic!("Unsupported API"),
    };

    let input_langs = Input::<String>::with_theme(&terminal_theme)
        .with_prompt("Language bank (comma-separated)")
        .default(translator.get_supported_langs().join(","))
        .interact_text()
        .unwrap_or_default();
    let input_langs_vec = input_langs
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let input_rounds = Input::<usize>::with_theme(&terminal_theme)
        .with_prompt("Rounds to mangle")
        .default(20)
        .interact_text()
        .unwrap_or_default();

    let input_delay = Input::<u64>::with_theme(&terminal_theme)
        .with_prompt("API call cool-down (in milliseconds)")
        .default(1000)
        .interact_text()
        .unwrap_or_default();

    println!("[INFO] Configuration done.");

    loop {
        let input_text = Input::<String>::with_theme(&terminal_theme)
            .with_prompt("Text to mangle")
            .interact()
            .unwrap_or_default();
        let input_orig_lang = Input::<String>::with_theme(&terminal_theme)
            .with_prompt("Original language of the text")
            .interact_text()
            .unwrap_or_default();

        let langs = get_random_lang_path(&input_orig_lang, &input_langs_vec, input_rounds);

        println!("{}", "[INFO] Processing...");
        let mangled = mangle(&translator, &input_text, &langs, input_delay);
        if let Err(e) = mangled {
            println!("[ERROR] {}", e.message);
        } else {
            println!("[OK] {}", mangled.unwrap());
        }
        println!("{}", "[INFO] Done.");
    }
}
