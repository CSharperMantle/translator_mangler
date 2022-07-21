# translator_mangler

| [English (en)](README.md) | **中文 (zh-CN)** |

用翻译服务让你的文本在不同语言间反复横跳，之后将其转换回源语言。

## 支持的翻译API

* [x] [百度翻译](https://api.fanyi.baidu.com/product/111)， `Baidu`
* [x] [有道 AI](https://ai.youdao.com/product-fanyi-text.s)， `Youdao`
* [ ] [谷歌翻译](https://cloud.google.com/translate/docs/reference/rest/v2/translate)， `Google Cloud`

*注：打勾的API已经经过详尽的测试。*

## 使用本软件

### 安装

你可以任选一种方法：

1. 从[Releases](https://github.com/CSharperMantle/translator_mangler/releases)页面下载最新的预编译包；
2. 自行从源代码构建。

### 运行

请按照终端的指引操作。注意，某些API可能需要在相对应的组织网站上创建账户才能够使用，某些API是收费的。详情请参考上一节中的链接。

以下是一次典型的使用百度API的运行样例。

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

## 开发

本软件使用纯Rust编写。在配置好Rust开发环境后，即可直接克隆本repo，开始开发。

```sh
git clone --depth=1 -- https://github.com/CSharperMantle/translator_mangler.git
cd translator_mangler

# 运行Debug版
cargo run
# 运行Release版
cargo run -r
```

## 许可协议

详见[LICENSE](LICENSE)。
