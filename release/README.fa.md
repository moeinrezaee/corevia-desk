# ابزارهای انتشار Corevia Desk

مسیر انتشار برای هر پروفایل، سیستم‌عامل و معماری یک بسته‌ی Rust به‌همراه Nuxt
ایستا می‌سازد. در macOS آن را به `Corevia Desk.app` واقعی تبدیل می‌کند؛ سپس
Archive نسخه‌دار، Manifest و checksum مستقل ایجاد می‌کند. امضای دیجیتال اختیاری
است و CI آن را اجباری نمی‌کند. Publisher قبل از ارسال وجود مسیر را بررسی می‌کند
و فایل نسخه‌دار قبلی را هرگز جایگزین نمی‌کند.

Secretهای لازم فقط از مخزن امن CI می‌آیند:

- `NEXUS_URL`، `NEXUS_REPOSITORY`، `NEXUS_USER` و `NEXUS_PASSWORD` برای انتشار.

هیچ Secretی نباید در Source، Manifest، Log یا Metadata مخزن باشد. انتشار واقعی
به Runner بومی Windows، macOS و Linux نیاز دارد. مسیر محلی macOS arm64 قابل Build
است؛ Build ویندوز و در صورت امکان Linux فقط روی Runner بومی بررسی می‌شود.

برای Build محلی macOS فقط وقتی `COREVIA_SKIP_NPM_INSTALL=1` را بگذار که پوشه‌ی
موجود `web/node_modules` معتبر و از قبل نصب شده باشد. CI این گزینه را نمی‌گذارد
و همیشه نصب پاک انجام می‌دهد.

برای Build آزمایشی که نباید بسته‌ی موجود را جایگزین کند،
`COREVIA_RELEASE_OUTPUT_DIR` را به یک مسیر اختصاصی زیر `.work` مخزن تنظیم کن؛
مثلاً `.work/native-smoke`. Build مسیر بیرون از `.work` را رد می‌کند و اگر این
گزینه تنظیم نشود، مسیر عادی `release-output` بدون تغییر باقی می‌ماند.

بسته‌ی macOS پس از ساخت با شناسه‌ی داخل `Info.plist` امضا می‌شود تا مجوزهای
حریم خصوصی به همان برنامه وصل شوند. برای تست محلی، امضای موقت پیش‌فرض کافی است؛
اما پس از Build دوباره، macOS ممکن است مجوز را دوباره بخواهد. برای حفظ هویت بین
Buildها، `COREVIA_MACOS_SIGNING_IDENTITY` را به گواهی امضای ثابت تنظیم کن. انتشار
عمومی macOS همچنان به گواهی Developer ID و فرایند Notarization نیاز دارد.

در CI یک خروجی Docker لینوکس فقط برای Full Desk هم روی Runner لینوکسِ دارای Docker
ساخته می‌شود. فایل image و اطلاعات آن به‌عنوان Artifact خروجی لینوکس تحویل می‌شود
و خود image هم در رجیستری خصوصی Docker روی Nexus منتشر می‌شود؛
نسخه‌ی Customer Support عمداً داخل این کانتینر نیست. هنگام اجرا می‌توان
`COREVIA_ID_SERVER`، `COREVIA_RELAY_SERVER`، `COREVIA_API_SERVER` اختیاری،
`COREVIA_SERVER_KEY` و `COREVIA_DESK_PORT` را از Environment تنظیم کرد. مقدار آخر
پورت دسترسی مستقیم را تعیین می‌کند و باید با `-p` به پورت میزبان نگاشت شود. هیچ
کلید شبکه‌ای داخل Image یا Source ذخیره نمی‌شود.
