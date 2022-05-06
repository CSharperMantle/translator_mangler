# translator_mangler

Mangle your input text by translating it over and over again.

## Supported translation APIs

* [x] [百度翻译 (Baidu Translate)](https://api.fanyi.baidu.com/product/111)
* [ ] [Google Cloud Translation](https://cloud.google.com/translate/docs/reference/rest/v2/translate)

More APIs are coming soon.

## Using `translator_mangler`

Run the app and follow the instructions in the terminal.

Below it a typical run with Baidu Translation API:

```text
PS > .\translator_mangler.exe
PS > cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 1.61s
     Running `target\debug\translator_mangler.exe`
[INFO] Welcome to translator_mangler!
✔ Select translate API · Baidu
✔ API key for Baidu Translation API · ********************
✔ App ID for Baidu Translation API · *****************
✔ Language bank (comma-separated) · en,zh,wyw,jp,fra,kor,th,pt,el,bul,ru,ara,spa,rom
✔ Rounds to mangle · 25
✔ API call cool-down (in milliseconds) · 150
[INFO] Configuration done.
✔ Text to mangle · The quick fox jumps over the lazy dog. The price of the shirt is 9.15 pounds.
✔ Original language of the text · en
[INFO] Processing...
[OK] The fox put the dog on his shirt and sold it for $25.
[INFO] Done.
✔ Text to mangle · 先帝创业未半而中道崩殂，今天下三分，益州疲弊，此诚危急存亡之秋也。
✔ Original language of the text · zh
[INFO] Processing...
[OK] 第一个日历不能分为两部分。现在这是一个没有良心的角色
[INFO] Done.
? Text to mangle › ^C
PS > 
```

### Getting access to Baidu Translation API

You will need to obtain an API key and an App ID from [Baidu Open Platform](https://fanyi-api.baidu.com/product/11). The service is free for personal use.

After obtaining the API key and App ID, you can use them in the `translator_mangler`.

## License

```plain
BSD 3-Clause License

Copyright (c) 2022, Rong "Mantle" Bao.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of Google Inc. nor the names of its contributors may be
   used to endorse or promote products derived from this software without
   specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
```
