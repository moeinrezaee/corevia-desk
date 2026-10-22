[English](./corevia-desk-problem-history.md)

# تاریخچه‌ی مشکل‌های Corevia Desk

آخرین به‌روزرسانی: ۲۰۲۶-۰۹-۱۰
وضعیت: سابقه‌ی فعال
مالک: Platform Engineering

این سند تاریخچه‌ی فشرده و قابل ارجاع خطاهایی است که قبلاً باعث Drift یا اعلام تکمیل اشتباه شدند. Agent بعدی باید قبل از تکرار عیب‌یابی آن را بخواند.

## هدف

جلوگیری از تکرار بررسی‌های حل‌شده و وصل‌کردن هر مشکل باز به یک گیت پذیرش مشخص.

## محدوده

این سابقه مسیر فعال Rust/WebView، Smoke اتصال ID/Relay، مرز MCP، بسته‌بندی بومی و عملیات انتشار کنترل‌شده‌ی Corevia Desk را پوشش می‌دهد.

## پیش‌نیاز

قبل از عیب‌یابی اتصال یا انتشار، وضعیت فعلی، ماتریس پذیرش و handoff را بخوان.

## پیکربندی

تنظیمات سرور فقط Runtime هستند. این سند فقط نام تنظیمات و پورت‌ها را دارد و مقدار Secret را ندارد.

## اجرا و استفاده

از جدول‌ها برای تشخیص حل‌شده، باز، غیرمسدودکننده یا Regression استفاده کن. ردیف جدید فقط بعد از ثبت Evidence اضافه شود.

## اعتبارسنجی

هر وضعیت را با Evidence فعلی و ماتریس پذیرش تطبیق بده. Plan یا قرارداد سورس به‌تنهایی حادثه را نمی‌بندد.

## رفع اشکال

ابتدا Process محصول روی هر دو مقصد، سپس شناسه و ID/Relay، و بعد مسیر Session/Frame/Input بومی را بررسی کن. IP، RDP یا Fixture مرورگر جایگزین نیست.

## مشکل‌های حل‌شده

| ۴۰۳ Runner ویندوز | منبع HTTPS ورودی ویندوز در Traefik تولیدی با Values جدید reconcile نشده بود | اجرای رسمی Traefik و ورودی فایروال با همان منبع انجام شد؛ API تأیید Runner در هر دو مسیر ۲۰۰ و `gitlab-runner verify` با `rc=0` Readback شد | حل‌شده؛ Runner آنلاین است |

| مشکل | علت | راه‌حل | وضعیت |
| --- | --- | --- | --- |
| باینری مک به‌دلیل نبود `libsciter.dylib` Crash می‌کرد | Launcher قدیمی Sciter هنوز اجرا می‌شد | Launcher فعال به WebView بومی Rust با `tao-wry` و Assetهای Nuxt منتقل شد | حل‌شده؛ Sciter خارج از Scope فعال است |
| Flutter/Dart به‌عنوان وابستگی دسکتاپ فرض شد | Layout قدیمی RustDesk با معماری Corevia اشتباه گرفته شد | مسیر فعال و CI روی Rust + Nuxt/WebView قفل شد و Flutter/Dart فقط حفظی ماند | با قرارداد Scope حل شد |
| درخواست ویندوز قبل از تحویل Native Manager در Pending می‌ماند | مسیر Delivery به State قدیمی قبل از Login وابسته بود | Delivery نشست بومی Rust و Action تأیید Nuxt اصلاح و روی Full Desk ویندوز تست شد | در Build تست‌شده حل شد |
| اولین فریم نشست Pending را می‌بست | Callback اولین فریم به‌اشتباه `close_success()` بود | اولین فریم حالا نشست را Active می‌کند و `first-frame-ready` ثبت می‌شود | حل‌شده |
| درخواست تکراری همان شناسه نشست دوم می‌ساخت | Lookup رجیستری دیرتر از ساخت Connection انجام می‌شد | Connection Pending/Active موجود قبل از ساخت نشست دوم Reuse می‌شود | حل‌شده و زنده تأیید شد |
| Capture در RDP پایدار نبود | Process ویندوز داخل نشست RDP اجرا می‌شد | با WinRM نشست به Console ویندوز منتقل شد؛ RDP مسیر اتصال محصول نبود | برای محیط Smoke حل شد |
| یک Bundle محلی Customer Support مک metadata محدود اما native binary از نوع Full Desk داشت | Buildهای Full Desk و Customer Support یک Target مشترک Cargo داشتند و باینری قدیمی می‌توانست وارد Bundle پروفایل جدید شود | Targetهای Cargo در Release بر اساس profile/platform/architecture جدا شدند، بسته‌بندی Skip-build رد می‌شود، Bundle Customer Support دوباره ساخته و با profile مورد انتظار Native Readback شد | در مسیر Release محلی حل شد؛ Install و GUI زنده هنوز باز است |
| دو Readback هم‌زمان image در Bridge یک گزارش مشترک می‌ساختند | شناسهٔ اجرای گزارش فقط ثانیه را شامل می‌شد | شناسهٔ اجرا با PID همان Process یکتا شد و Readback موازی Node و Debian مستقل و صحیح برگشت | حل‌شده؛ تست موازی و Evidence ثبت شد |

## مشکل‌های باز یا غیرمسدودکننده

| مشکل | اثر | برخورد فعلی |
| --- | --- | --- |
| دو نشست Remote هم‌زمان با دو مقصد مستقل هنوز Evidence نشده‌اند | درخواست دوم با همان Peer ID طبق Guard محصول نشست موجود را reuse می‌کند؛ namespace بومی و Peer ID دو پروفایل اکنون جدا هستند | دو درخواست مستقل `/r` با Peer ID به مقصدهای Windows و Customer Support هم‌زمان تأیید و Active شدند؛ هر دو `direct:false` و `secured:true` را بیش از ۲۰ ثانیه Readback کردند و سپس تمیز بسته شدند | حل‌شده؛ گیت اصلی P1 بسته است. Readback تصویری Selector و نصب جانبی Windows همچنان گیت مشاهده/تحویل P1.5 هستند |
| Copy کاربر روی macOS بعد از دریافت Clipboard راه دور با علامت مالکیت باقی‌مانده مسدود می‌شد | بعضی برنامه‌ها `dyn.com.rustdesk.owner` را هنگام Copy نگه می‌دارند و مسیر قبلی فقط علامت را می‌دید | تفکیک دقیق Host/Client و مقایسهٔ `NSPasteboard.changeCount` با آخرین Write راه دور اضافه شد؛ تست واحد، Build و انتقال واقعی دوطرفه با marker زنده PASS است | حل‌شده؛ Evidence P1 به‌روز شد |
| ماتریس کامل Install/Connect/UI نسخه Customer Support روی دو سیستم بومی کامل Evidence نشده | گیت P1.5 باز است | Build بومی هر دو Target، artifact ویندوز، process launch و ACL/MCP headless ثبت شده؛ Install جانبی، UI تصویری و Readback محصولی باقی است |
| MCP برای هر دو پروفایل مک باید از Read امن فراتر می‌رفت | بدون Mutation، Approval، Audit و Deny readback، برابری با ACL محصول ثابت نمی‌شد | کاتالوگ ۲۵/۹، عملیات مجاز، Approval با مرجع، Audit، عدم اجرای نشست ناموجود و Denyهای پروفایلی/حساس از Native MCP خوانده شد | در محیط محلی حل‌شده؛ P2 بسته است |
| فضای دیسک مقصد Windows برای اجرای تلاش نصب تمام شد | build/cacheهای دقیق تلاش‌های قبلی چند گیگابایت فضا گرفته بودند و WinRM Taskها ناقص می‌شدند | فقط مسیرهای دقیق build/cache مرتبط پاک و حدود ۴ گیگابایت فضا آزاد شد؛ نصب Full Desk دست‌نخورده ماند | مشکل فضا حل شد؛ انتقال archive Customer Support هنوز بلاکر مستقل است |
| انتقال archive Customer Support Windows از WinRM stream قابل اتکا نبود | streamهای بزرگ قبل از رسیدن کامل فایل قطع یا truncate شدند و archive قابل اعتبارسنجی نبود | فایل‌های ناقص و Taskهای دقیق پاک شدند؛ نصب ناقص ادعا نشد و مسیر رسمی Executor/Artifact باید استفاده شود | P1.5 باز؛ bypass دستی مجاز نیست |
| اجرای معتبر CI روی commit فعلی هنوز Readback نشده است | imageهای پایهٔ CI اکنون با digest immutable و `linux/amd64` از مسیر رسمی Resolver/Bridge در Nexus هستند، اما اجرای موجود روی commit قدیمی `2fc0e57f` پس از Pull موفق Node در `npm ci` با lockfile ناسازگار شکست خورد | Source Resolver، Publish و Readback رسمی هر دو image ثبت شد؛ اجرای قدیمی Retry نشد و image نامرتبط جایگزین نشد | P4 باز؛ Executor اجرای جدید روی commit فعلی، Artifact و Readback انتشار لازم است |
| انتشار حاکمیت‌شدهٔ شاخهٔ کاری در دسترس نبود | اجرای CI روی commit فعلی را از دیدن تغییرات محلی بازمی‌داشت | Pattern و Executor جدید `gitlab-branch.publish` در Bridge اکنون worktree تمیز، ancestor بودن commit و Readback دقیق commit را کنترل می‌کنند؛ Plan سبز است و Apply منتظر commit تمیز است | Apply انتشار شاخه، سپس Sync fast-forward-only به‌ترتیب `codex → develop → main` و اجرای CI فعلی |
| پورت جانبی Audit/API یعنی ۲۱۱۱۴ در بررسی قبلی Timeout داشت | مسیر ID/Relay و کنترل را متوقف نمی‌کند اما مشکل سرویس جداست | جداگانه پیگیری شود و معیار اتصال محصول گسترده نشود |
| Publish و Readback محافظت‌شده CI/Nexus محلی ادعا نشده | تکمیل Release را متوقف می‌کند، نه P0 محلی را | فقط با Runner بومی و Patternهای Bridge اجرا شود |
| Executor اختصاصی نصب/سلامت/به‌روزرسانی/Rollback محصول در Bridge وجود ندارد | نصب و به‌روزرسانی عملیاتی حاکمیت‌شده باز است | `extension-needed` بماند و Mutation دستی انجام نشود |

## قواعد عیب‌یابی

- اتصال IP، هم‌شبکه‌بودن، RDP یا Fixture مرورگر جای تست RustDesk با Peer ID از ID/Relay نیست.
- قبل از عیب‌یابی سرور، روشن‌بودن محصول روی مبدا و مقصد را چک کن.
- برای عملیات ویندوز از WinRM استفاده کن؛ RDP مسیر تست محصول نیست.
- MCP همراه اختیاری و حاکمیت‌شده است و بعد از کارکرد مسیر محصول بررسی می‌شود.
- Secretها را در Evidence نیاور و فقط نام تنظیم Runtime را ذکر کن.

## Evidence

- [وضعیت فعلی](../evidence/current-status.fa.md)
- [مدرک CD-8 تا CD-12](../evidence/corevia-desk-phase-8-12.fa.md)
- [ماتریس پذیرش](../specs/acceptance.fa.md)

## تاریخچه تغییرات

- ۲۰۲۶-۰۹-۱۷: بلاکر ۴۰۳ Runner ویندوز با reconcile رسمی Traefik و readback HTTPS/API حل شد؛ هیچ تغییر دستی روی Runner یا WireGuard انجام نشد. گیت‌های CI، Artifact و UI ویندوز مستقل باقی هستند.

- ۲۰۲۶-۰۹-۱۰: Source Resolver/Publish/Readback رسمی دو image پایهٔ CI با digest immutable و معماری `linux/amd64` انجام شد؛ هر دو digest مقصد Nexus دقیقاً Readback شدند. اجرای Pipeline موجود بعد از Pull موفق Node روی commit قدیمی `2fc0e57f` به‌دلیل lockfile ناسازگار شکست خورد و بلاکر از image supply به اجرای جدید/Artifact منتقل شد.

- ۲۰۲۶-۰۹-۰۹: Readback رسمی تکرار شد؛ Source Audit سبز ماند، اما imageهای پایهٔ CI در Nexus همچنان غایب‌اند و Pipeline پیش از تست‌ها متوقف می‌شود. هیچ Retry یا Mutation دستی انجام نشد.
- ۲۰۲۶-۰۹-۰۸: Source Audit در بازبینی تازه Drift ناشی از تغییر سورس Bridge را دید؛ Catalog Write رسمی و Audit فوری بعدی سبز شد. Readback imageهای CI همچنان نبود `node:22-bookworm` و `debian:trixie-slim` را نشان داد و Pipeline پیش از تست‌ها متوقف ماند.
- ۲۰۲۶-۰۹-۰۵: مشکل وابستگی Sciter، اشتباه Scope Flutter، Delivery بومی، اولین فریم، نشست تکراری، RDP و گیت‌های انتشار یک‌جا ثبت شدند.
- ۲۰۲۶-۰۹-۰۶: Windows native Customer Support، process launch و ACL/MCP headless ثبت شد؛ نبود Executor رسمی نصب و نبود UI Automation معتبر، P1.5 را باز نگه داشت.
- ۲۰۲۶-۰۹-۰۷: ناسازگاری profile در بسته‌بندی مک ثبت و با Targetهای native جدا و Native Readback امن هر دو پروفایل اصلاح شد؛ Executor رسمی نصب و GUI زنده همچنان باز هستند.
- ۲۰۲۶-۰۹-۰۷: انتقال واقعی Clipboard دوطرفه با marker زنده PASS شد؛ در تلاش میانی درخواست دوم با همان Peer ID طبق Guard نشست موجود را reuse کرد و گیت مستقل‌بودن مقصد دوم موقتاً باز ماند؛ اجرای بعدی همان روز گیت P1 را بست.
- ۲۰۲۶-۰۹-۰۷: گیت نشست هم‌زمان P1 با دو مقصد مستقل Windows و Customer Support بسته شد؛ هر دو نشست `/r` با Peer ID تأییدشده، Active، Relay و امن بیش از ۲۰ ثانیه پایدار ماندند و تمیز بسته شدند. نصب جانبی/GUI ویندوز، Secret مثبت، Linux/Docker و CI/Nexus همچنان باز هستند.
- ۲۰۲۶-۰۹-۰۷: Pipeline رسمی در `verify:contracts` پیش از تست‌ها به‌علت نبود `node:22-bookworm` متوقف شد و Readback رسمی Nexus نبود `node:22-bookworm` و `debian:trixie-slim` را تأیید کرد؛ image داخلی نامرتبط جایگزین نشد و هیچ Retry یا Mutation دستی انجام نشد.
- ۲۰۲۶-۰۹-۰۷: ماتریس کامل Native MCP برای هر دو پروفایل Readback شد و P2 در محیط محلی بسته شد؛ کمبود فضای مقصد با پاک‌سازی دقیق cache/build رفع شد، اما انتقال archive Customer Support از stream WinRM همچنان به Executor رسمی نیاز دارد.

## مالکیت

Platform Engineering.
