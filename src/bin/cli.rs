use inquire::error::InquireError;

use translator_mangler::get_random_lang_path;
use translator_mangler::mangle;
use translator_mangler::Translator;
use translator_mangler::TranslatorBaidu;
use translator_mangler::TranslatorGoogleCloud;
use translator_mangler::TranslatorYoudao;

fn prompt_baidu_api() -> inquire::error::InquireResult<Box<dyn Translator>> {
    let input_api_key = inquire::Text::new("API key for Baidu API?").prompt()?;
    let input_app_id = inquire::Text::new("App ID for Baidu API?").prompt()?;

    Ok(Box::new(TranslatorBaidu::new(
        &input_app_id,
        &input_api_key,
    )))
}

fn prompt_google_cloud_api() -> inquire::error::InquireResult<Box<dyn Translator>> {
    let input_api_key = inquire::Text::new("API key for Google Cloud API?").prompt()?;

    Ok(Box::new(TranslatorGoogleCloud::new(&input_api_key)))
}

fn prompt_youdao_api() -> inquire::error::InquireResult<Box<dyn Translator>> {
    let input_app_key = inquire::Text::new("App key for Youdao AI API?").prompt()?;
    let input_app_secret = inquire::Text::new("App secret for Youdao AI API?").prompt()?;

    Ok(Box::new(TranslatorYoudao::new(
        &input_app_key,
        &input_app_secret,
    )))
}

fn main() -> inquire::error::InquireResult<()> {
    println!("[i] Welcome to translator_mangler!");

    let api_choices = vec!["Baidu", "Google Cloud", "Youdao AI"];
    let input_api_choices = inquire::Select::new("Back-end API?", api_choices)
        .with_help_message("Choose the back-end translation API you want to use")
        .prompt()?;
    let translator = match input_api_choices {
        "Baidu" => prompt_baidu_api()?,
        "Google Cloud" => prompt_google_cloud_api()?,
        "Youdao AI" => prompt_youdao_api()?,
        _ => {
            return Err(InquireError::InvalidConfiguration(
                "Back-end API".to_string(),
            ))
        }
    };

    let input_langs = inquire::Text::new("Language bank?")
        .with_help_message("Choose the languages you want to use in mangling, separated by comma")
        .with_default(&translator.get_supported_langs().join(","))
        .prompt()?;
    let input_langs_vec = input_langs
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let input_rounds = inquire::CustomType::<usize>::new("Rounds to mangle?")
        .with_help_message("Enter the number of rounds you want to translate")
        .with_default(20)
        .prompt()?;

    let input_delay = inquire::CustomType::<u64>::new("API cool-down?")
        .with_help_message("Enter the milliseconds between each API call")
        .with_default(1000)
        .prompt()?;

    let input_preview_plan = inquire::CustomType::<bool>::new("Preview plan?")
        .with_help_message("Enter whether you want to preview mangling plans before running")
        .with_default(true)
        .prompt()?;

    println!("[+] Configuration done.");

    loop {
        let input_text = inquire::Text::new("Text to mangle?")
            .with_help_message("Enter the text you wish to mangle")
            .prompt()?;
        let input_orig_lang = inquire::Text::new("Original language?")
            .with_help_message("Enter the original language of the text you entered")
            .prompt()?;

        let langs = get_random_lang_path(&input_orig_lang, &input_langs_vec, input_rounds);

        if input_preview_plan {
            println!(
                "[i] Mangling plan: {}",
                langs
                    .iter()
                    .map(|pair| format!("{}->{} ", pair.from_lang, pair.to_lang))
                    .reduce(|acc, current| acc + &current)
                    .unwrap_or_default()
            )
        }

        println!("[i] Processing...");
        let mangled = mangle(translator.as_ref(), &input_text, &langs, input_delay);
        match mangled {
            Ok(result) => println!("[+] {}", result),
            Err(e) => println!("[!] {}", e.message),
        }
    }
}
