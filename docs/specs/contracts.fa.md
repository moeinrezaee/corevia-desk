[English](./contracts.md)

# قراردادهای Corevia Desk

آخرین به‌روزرسانی: ۲۰۲۶-۱۰-۰۵
وضعیت: فعال
مالک: Platform Engineering

## هدف

مرزهایی را تعریف می‌کند که اجازه نمی‌دهد UI، MCP، بسته‌بندی و عملیات از هسته‌ی محصول Rust جدا یا منحرف شوند.

## Rust Core و ACL

- `src/corevia_acl.rs` تنها مرز مجوز برای Nuxt، MCP و Entry Pointهای بومی پوشش‌داده‌شده است.
- Full Desk از پروفایل پیش‌فرض و Customer Support از `corevia-customer-support` استفاده می‌کند.
- عملیات حساس به Approval Reference نیاز دارند و Audit شماره‌دار و بدون Secret تولید می‌کنند.
- `system-command` برای هر دو پروفایل رد می‌شود.
- UI به Rust سطح پایین، Shell، URL، SQL، Repository یا Command آزاد دسترسی ندارد.

## قرارداد Nuxt/WebView

- Launcher فعال `src/main.rs` به `src/webview.rs` است.
- Host assetهای Nuxt را با پروتکل خصوصی `corevia://` باز می‌کند.
- سطح نام‌دار `window.__COREVIA_DESK_BRIDGE__` به Dispatch مربوط به ACL Rust وصل است.
- Fallback مرورگر می‌تواند Fixture و `local-agent-required` نشان دهد، اما نباید قابلیت بومی را واقعی اعلام کند.
- پاسخ `accepted` به‌تنهایی اثبات اجرای عملیات نیست؛ اگر `data.executed` برابر `false` باشد، UI باید خطای قابل‌فهم بدهد و حالت/داده را خوش‌بینانه تغییر ندهد. MCP نیز همان مورد را به‌صورت خطای ابزار برمی‌گرداند.
- ترتیب Atomic Design این است: `Atoms -> Molecules -> Organisms -> Templates -> Layouts/Pages`.

## قرارداد اتصال محصول

- اتصال پایه از خود محصول روی هر دو مقصد شروع می‌شود.
- مقصد با شناسه‌ی RustDesk انتخاب می‌شود.
- ID Server، Relay Server و Key از Runtime خوانده می‌شوند و هرگز داخل سند یا Log کپی نمی‌شوند.
- پذیرش شامل تأیید، Active شدن، تصویر زنده، کنترل ماوس و کیبورد، بستن و اتصال دوباره است.
- MCP فقط از سطح حاکمیت‌شده تأیید یا مشاهده می‌کند و جای مسیر نشست محصول نیست.

## قرارداد MCP

- Transport از نوع stdio است و فقط `initialize`، `tools/list` و `tools/call` را دارد.
- Full Desk و Customer Support Catalog مؤثر جدا دارند.
- MCP از همان ACL Rust و قواعد Approval استفاده می‌کند. ابزارها مرجع تأیید را از
  فراخواننده نمی‌گیرند؛ اگر کلاینت قابلیت Form Elicitation را اعلام کند، Adapter
  برای هر درخواست حساس جداگانه `elicitation/create` می‌فرستد. رد، لغو، timeout،
  پروتکل قدیمی یا نبود قابلیت باعث Deny می‌شود.
- پاسخ پذیرفته‌شده به اثبات HMAC داخلی، محدود به مرجع تصادفی، عملیات و payload
  تبدیل می‌شود؛ Rust آن را اعتبارسنجی و یک‌بارمصرف می‌کند. این اثبات بخشی از ابزار
  عمومی نیست و متن دلخواه فراخواننده جای آن را نمی‌گیرد. پذیرش نهایی نیازمند
  آزمون با کلاینت MCP واقعیِ دارای elicitation و تکمیل ماتریس اجراگرهاست.
- MCP Proxy آزاد Rust، Shell، System Command، SQL، URL یا Repository نیست.
- مجوز ACL به معنی اجرای موفق نیست؛ اگر پل بومی `data.executed: false` برگرداند، نتیجهٔ MCP باید خطا باشد و موفقیت ظاهری گزارش نشود.
- ابزار ارسال پیام Full Desk برای هر پیام تأیید جداگانه می‌گیرد و متن پاک‌سازی‌شده را در درخواست تأیید نشان می‌دهد.

## قرارداد انتشار

- هویت Artifact برابر است با `corevia-desk/<profile>/<version>/<platform>/<architecture>/`.
- پروفایل‌ها `customer-support` و `full-desk` هستند.
- Customer Support برای macOS و Windows است.
- Full Desk برای macOS، Windows، Linux و Image Docker لینوکس است.
- Archive، Manifest و Checksum اجباری هستند؛ امضا اختیاری است و CI آن را اجباری نمی‌کند.
- مسیر نسخه‌دار Nexus تغییرناپذیر است و نباید Overwrite شود.

## مرز Corevia Bridge

Bridge مالک نصب سورس‌محور، سلامت، به‌روزرسانی، Rollback، تنظیمات Runtime و عملیات Nexus است. مالک رفتار Rust، قواعد UI، مجوزهای MCP یا منطق محصول نیست. اگر Executor اختصاصی محصول وجود نداشته باشد، وضعیت `extension-needed` است و Mutation دستی جایگزین معتبر نیست.

## تاریخچه‌ی تغییر

- ۲۰۲۶-۱۰-۰۵: تأیید میزبانی‌شدهٔ MCP با Form Elicitation و اثبات داخلیِ یک‌بارمصرف اضافه شد؛ آزمون کلاینت واقعی و برخی اجراگرها همچنان بازند.
- ۲۰۲۶-۰۹-۰۵: مرزهای ACL Rust، WebView، ID/Relay، MCP، انتشار و Bridge یکپارچه شد.

## مالکیت

Corevia Desk مالک این قراردادهای محصول است. Schema و حاکمیت مشترک از Corevia Standards و اجرای عملیاتی از Corevia Bridge می‌آید.
