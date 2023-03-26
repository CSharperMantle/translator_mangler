# translator_mangler

| **English (en)** | [中文 (zh-CN)](README-zh.md) |

Mangle your input text by translating it over and over again.

## Supported translation APIs

* [x] [百度翻译 (Baidu Translate)](https://api.fanyi.baidu.com/product/111), `Baidu`
* [x] [有道 AI (Youdao AI Translation)](https://ai.youdao.com/product-fanyi-text.s), `Youdao`
* [ ] [Google Cloud Translation](https://cloud.google.com/translate/docs/reference/rest/v2/translate), `Google Cloud`

*Note: Only ticked APIs are fully tested.*

## Using `translator_mangler`

### Installation

You may:

1. Download one of the pre-built binaries on [Releases](https://github.com/CSharperMantle/translator_mangler/releases) page,
2. Or build your own version according to the [Development](#development) section.

### Run

Follow the instructions displayed in the terminal.

You may need to register or purchase a subscription to use the APIs. Please refer to each API's manual for more information.

Below it a typical run with Baidu Translation API on PowerShell:

```text
PS > .\translator_mangler.exe
[INFO] Welcome to translator_mangler!
? Back-end API? Baidu
? API key for Baidu API? ********************
? App ID for Baidu API? *****************
? Language bank? zh,en,yue,wyw,jp,kor,fra,spa,th,ara,ru,pt,de,it,el,nl,pl,bul,est,dan,fin,cs,rom,slo,swe,hu,cht,vie
? Rounds to mangle? 20
? API cool-down? 150
[INFO] Configuration done.
? Text to mangle? The quick fox jumps over the lazy dog. The price of the shirt is 9.15 pounds.
? Original language? en
[INFO] Processing...
[OK] The shirts are 165 and 169.
[INFO] Done.
? Text to mangle? 先帝创业未半而中道崩殂，今天下三分，益州疲弊，此诚危急存亡之秋也。
? Original language? zh
[INFO] Processing...
[OK] 第一位皇帝什么也没失去，但失败了。
[INFO] Done.
? Text to mangle? ^C
[Enter the text you wish to mangle]
Error: OperationInterrupted

PS > 
```

## Development

The project is written in pure Rust. You can simply clone the repository and Cargo will handle all the rest when you have your Rust toolchain ready.

```sh
git clone --depth=1 -- https://github.com/CSharperMantle/translator_mangler.git
cd translator_mangler

# Run development version
cargo run
# or release version
cargo run -r
```

## License

See [LICENSE](LICENSE).
