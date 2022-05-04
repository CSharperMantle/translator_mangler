# translator_mangler

Mangle your input text by translating it over and over again.

## Supported translation APIs

* [x] [百度翻译 (Baidu Translate)](https://api.fanyi.baidu.com/product/111)

More APIs are coming soon.

## Using `translator_mangler`

Run the app and follow the instructions in the terminal.

Below it a typical run with Baidu Translation API:

```text
PS > .\translator_mangler.exe
✔ Select translate API · Baidu
✔ API key for Baidu Translation API · ********************
✔ App ID for Baidu Translation API · *****************
✔ Language list to mangle (separated by comma) · en,zh,wyw,jp,fra,kor,th,pt,el,bul,ru,ara,spa,rom
✔ Number of rounds to mangle · 20
✔ Delay between rounds (in milliseconds) · 500
✔ Text to mangle · The quick fox jumps over the lazy dog. The price of the shirt is 9.25 dollars.
✔ Original language of the text · en
[Processing]
[Result] The fox put the dog on his shirt and sold it for $25.
[Done]
✔ Text to mangle · I used to travel by air a great deal when I was a boy.
✔ Original language of the text · en
[Processing]
[Result] I'm still young.
[Done]
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
