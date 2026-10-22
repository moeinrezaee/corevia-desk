<p dir="rtl" align="center">
  <img src="../res/logo-header.svg" alt="RustDesk - Your remote desktop"><br>
  <a href="#تصاویر-محیط-نرم افزار">تصاویر محیط نرم‌افزار</a> •
  <a href="#ساختار-پوشه-ها">ساختار</a> •
  <a href="#نحوه-ساخت-با-داکر">داکر</a> •
  <a href="#ساخت">ساخت</a> •
  <a href="#سرورهای-عمومی-رایگان">سرور</a>
</p>
<p align="center" dir="auto">[<a href="../README.md">English</a>] | [<a href="README-UA.md">Українська</a>] | [<a href="README-CS.md">česky</a>] | [<a href="README-ZH.md">中文</a>] | [<a href="README-HU.md">Magyar</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-ID.md">Indonesian</a>] | [<a href="README-FI.md">Suomi</a>] | [<a href="README-ML.md">മലയാളം</a>] | [<a href="README-JP.md">日本語</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-IT.md">Italiano</a>] | [<a href="README-RU.md">Русский</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] | [<a href="README-EO.md">Esperanto</a>] | [<a href="README-KR.md">한국어</a>] | [<a href="README-AR.md">العربي</a>] | [<a href="README-VN.md">Tiếng Việt</a>] | [<a href="README-GR.md">Ελληνικά</a>]</p>
<p dir="rtl" align="center"><b>برای ترجمه این سند (README)، <a href="https://github.com/rustdesk/rustdesk/tree/master/src/lang" dir="rtl">رابط کاربری RustDesk</a>، <a href="https://github.com/rustdesk/doc.rustdesk.com" dir="rtl">و مستندات آن</a> به زبان مادری شما به کمکتان نیازمندیم. </b></p>

با ما گفتگو کنید:  [Reddit](https://www.reddit.com/r/rustdesk) | [Twitter](https://twitter.com/rustdesk) | [Discord](https://discord.gg/nDceKgxnkV) | [YouTube](https://www.youtube.com/@rustdesk) 


[![RustDesk Server Pro](https://img.shields.io/badge/RustDesk%20Server%20Pro-%D9%88%DB%8C%DA%98%DA%AF%DB%8C%E2%80%8C%D9%87%D8%A7%DB%8C%20%D9%BE%DB%8C%D8%B4%D8%B1%D9%81%D8%AA%D9%87-blue)](https://rustdesk.com/pricing.html)

Corevia Desk یک محصول ریموت‌دسکتاپ با برند Corevia و بر پایه‌ی پروژه‌ی متن‌باز RustDesk است. یک Rust Core و دو پروفایل حاکمیت‌شده‌ی Full Desk و Customer Support دارد. رابط فعال دسکتاپ Nuxt است که داخل WebView بومی Rust اجرا می‌شود؛ سورس‌های قدیمی Flutter، Dart و Sciter فقط برای تاریخچه نگه داشته شده‌اند و وابستگی فعال Release نیستند.

Runtime پیش‌فرض از مسیر شناسه‌ی RustDesk و ID/Relay استفاده می‌کند؛ بنابراین دو دستگاه لازم نیست در یک شبکه باشند. مقادیر سرور از Runtime خوانده می‌شوند و در مستندات این مخزن ذخیره نمی‌شوند.

ما از مشارکت همه استقبال می کنیم. برای راهنمایی جهت مشارکت به[`docs/CONTRIBUTING.md`](CONTRIBUTING.md) مراجعه کنید.

[راست‌دسک چطور کار می کند؟](https://github.com/rustdesk/rustdesk/wiki/How-does-RustDesk-work%3F)

[دریافت نرم‌افزار](https://github.com/rustdesk/rustdesk/releases)

## وابستگی‌ها

مسیر فعال دسکتاپ به Rust، Node.js برای ساخت Assetهای Nuxt و Host بومی WebView نیاز دارد. Flutter، Dart و Sciter برای مسیر فعال دسکتاپ لازم نیستند. Build بومی Windows به ابزارهای MSVC و Windows SDK و هر سیستم‌عامل به کتابخانه‌های بومی Rust/WebView خودش نیاز دارد.

## مسیر ساخت

مسیر کنترل‌شده ابتدا Assetهای Nuxt را می‌سازد، آن‌ها را با Host بومی WebView Rust بسته‌بندی می‌کند، سطح پروفایل را بررسی می‌کند و Artifact دارای Checksum می‌سازد. برای جزئیات [راهنمای انتشار](../release/README.fa.md) و [فهرست مستندات](INDEX.fa.md) را بخوان.

برای لایه‌ی وب محلی:

```sh
npm --prefix web ci
npm --prefix web run lint:atomic
npm --prefix web run lint:profiles
npm --prefix web run build
COREVIA_DESK_PROFILE=customer-support npm --prefix web run build
```

برای تست فعال Rust:

```sh
cargo test --lib
```

برای بررسی دسکتاپ Corevia Desk از Launcherهای قدیمی Sciter یا Flutter استفاده نکن.

## مرجع قدیمی ساخت upstream (مسیر Corevia Desk نیست)

دستورهای باقی‌مانده‌ی زیر فقط برای تاریخچه نگه داشته شده‌اند و نباید برای Build دسکتاپ Corevia Desk استفاده شوند؛ مسیر [راهنمای انتشار](../release/README.fa.md) و Rust + Nuxt/WebView بومی را استفاده کن.

## نحوه ساخت بر روی لینوکس

### ساخت بر روی (Ubuntu 18 (Debian 10

```sh
sudo apt install -y g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake
```

### ساخت بر روی (Fedora 28 (CentOS 8

```sh
sudo yum -y install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libxdo-devel libXfixes-devel pulseaudio-libs-devel cmake alsa-lib-devel
```

### ساخت بر روی (Arch (Manjaro

```sh
sudo pacman -Syu --needed unzip git cmake gcc curl wget yasm nasm zip make pkg-config clang gtk3 xdotool libxcb libxfixes alsa-lib pipewire
```

### نرم افزار vcpkg را نصب کنید

```sh
git clone https://github.com/microsoft/vcpkg
cd vcpkg
git checkout 2023.04.15
cd ..
vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg
vcpkg/vcpkg install libvpx libyuv opus aom
```

### رفع ایراد libvpx (برای فدورا)

```sh
cd vcpkg/buildtrees/libvpx/src
cd *
./configure
sed -i 's/CFLAGS+=-I/CFLAGS+=-fPIC -I/g' Makefile
sed -i 's/CXXFLAGS+=-I/CXXFLAGS+=-fPIC -I/g' Makefile
make
cp libvpx.a $HOME/vcpkg/installed/x64-linux/lib/
cd
```

### ساخت

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
mkdir -p target/debug
wget https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so
mv libsciter-gtk.so target/debug
VCPKG_ROOT=$HOME/vcpkg cargo run
```

## مرجع قدیمی Docker upstream (Image Corevia Desk نیست)

این مخزن Git را دریافت کنید و کانتینر را به روش زیر بسازید

```sh
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
docker build -t "rustdesk-builder" .
```

سپس، هر بار که نیاز به ساخت نرم‌افزار داشتید، دستور زیر را اجرا کنید:

```sh
docker run --rm -it -v $PWD:/home/user/rustdesk -v rustdesk-git-cache:/home/user/.cargo/git -v rustdesk-registry-cache:/home/user/.cargo/registry -e PUID="$(id -u)" -e PGID="$(id -g)" rustdesk-builder
```

توجه داشته باشید که نخستین ساخت ممکن است به دلیل محلی نبودن وابستگی‌ها بیشتر طول بکشد. اما دفعات بعدی سریعتر خواهند بود. علاوه بر این، اگر نیاز به تعیین آرگومان های مختلف برای دستور ساخت دارید، می توانید این کار را در انتهای دستور ساخت و از طریق `<OPTIONAL-ARGS>` انجام دهید. به عنوان مثال، اگر می خواهید یک نسخه نهایی بهینه سازی شده ایجاد کنید، دستور بالا را تایپ کنید و در انتها  `release--` را اضافه کنید. فایل اجرایی به دست آمده در پوشه مقصد در سیستم شما در دسترس خواهد بود و می تواند با دستور:

```sh
target/debug/rustdesk
```

یا برای نسخه بهینه سازی شده دستور زیر را اجرا کنید:

```sh
target/release/rustdesk
```

لطفاً اطمینان حاصل کنید که این دستورات را از پوشه مخزن RustDesk اجرا می کنید، در غیر این صورت ممکن است برنامه نتواند منابع مورد نیاز را پیدا کند. همچنین توجه داشته باشید که سایر دستورات فرعی Cargo مانند `install` یا `run` در حال حاضر از طریق این روش پشتیبانی نمی شوند زیرا برنامه به جای سیستم عامل میزبان, در داخل کانتینر نصب و اجرا میشود.

## ساختار پوشه ها 

- **[libs/hbb_common](https://github.com/rustdesk/rustdesk/tree/master/libs/hbb_common)**: video codec, config, tcp/udp wrapper, protobuf, fs functions for file transfer, and some other utility functions
- **[libs/scrap](https://github.com/rustdesk/rustdesk/tree/master/libs/scrap)**: screen capture
- **[libs/enigo](https://github.com/rustdesk/rustdesk/tree/master/libs/enigo)**: platform specific keyboard/mouse control
- **[src/webview.rs](../src/webview.rs)**: Host بومی فعال برای رابط Nuxt
- **[src/corevia_acl.rs](../src/corevia_acl.rs)**: مرز مشترک مجوزهای Rust
- **[src/corevia_bridge.rs](../src/corevia_bridge.rs)**: Dispatch محدود پل Nuxt و Adapterهای محلی
- **[web](../web/)**: لایه‌ی فعال Nuxt با منبع `corevia-desk-ui`
- **[mcp](../mcp/)**: Adapter و Catalog حاکمیت‌شده‌ی MCP
- **[src/ui](https://github.com/rustdesk/rustdesk/tree/master/src/ui)**: GUI قدیمی حفظ‌شده، نه مسیر فعال Corevia Desk
- **[src/server](https://github.com/rustdesk/rustdesk/tree/master/src/server)**: audio/clipboard/input/video services, and network connections
- **[src/client.rs](https://github.com/rustdesk/rustdesk/tree/master/src/client.rs)**: start a peer connection
- **[src/rendezvous_mediator.rs](https://github.com/rustdesk/rustdesk/tree/master/src/rendezvous_mediator.rs)**: Communicate with [rustdesk-server](https://github.com/rustdesk/rustdesk-server), wait for remote direct (TCP hole punching) or relayed connection
- **[src/platform](https://github.com/rustdesk/rustdesk/tree/master/src/platform)**: platform specific code
- **[flutter](https://github.com/rustdesk/rustdesk/tree/master/flutter)**: سورس حفظ‌شده‌ی inherited و خارج از مسیر فعال Corevia Desk

## مستندات Corevia

- [فهرست مستندات](INDEX.fa.md)
- [وضعیت فعلی](evidence/current-status.fa.md)
- [ماتریس پذیرش](specs/acceptance.fa.md)
- [Handoff کانتکست بعدی](handoff/next-context.fa.md)

## تصاویر محیط نرم‌افزار

![image](https://user-images.githubusercontent.com/71636191/113112362-ae4deb80-923b-11eb-957d-ff88daad4f06.png)

![image](https://user-images.githubusercontent.com/71636191/113112619-f705a480-923b-11eb-911d-97e984ef52b6.png)

![image](https://user-images.githubusercontent.com/71636191/113112857-3fbd5d80-923c-11eb-9836-768325faf906.png)

![image](https://user-images.githubusercontent.com/71636191/135385039-38fdbd72-379a-422d-b97f-33df71fb1cec.png)
