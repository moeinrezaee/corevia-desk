[English](./roadmap.md)

# رودمپ Corevia Desk

## قفل جاری تحویل — ۲۰۲۶-۱۰-۰۸

باندل دقیق و جاری Customer Support پس از درخواست مجوز خودش و راه‌اندازی مجدد دوباره بررسی شد: macOS ورودی‌های ضبط صفحه/صدا را روشن نشان می‌دهد، اما رابط جاری هنوز capture=false می‌خواند. علت قطعی نیست. طبق تصمیم مالک، این نتیجهٔ خاص مک مانع ادامه نیست؛ رفتار مشترک تصویر/اتصال باید روی دسکتاپ آزمایشیِ مستقل Windows به‌عنوان شاهد جایگزین پاس شود. دسترسی فقط‌خواندنی WinRM به سرور Windows برقرار است، اما نشست Customer Support قطع است و گواهی RDP مورداعتماد نیست؛ دسکتاپ فعال و جدا برای پذیرش تصویری نداریم. C سرور حدود ۱۵ GiB آزاد دارد و volume شخصی ندارد، پس Docker نباید روی درایو سیستم نگهداری/اجرا شود.

Readback تازهٔ دادهٔ بومی: Devices برای Peerهای مک و Windows هرکدام یک ردیف دارد و زمان آخرین اتصال موفق تغییر نکرده. Sessions مقدار Active=0/دو اخیر را نشان می‌دهد؛ تلاش تازه برای Peer مک Failed است، نه اتصال موفق. Customer Support درخواست ورودی ندارد. [شواهد UI/دادهٔ P1](../evidence/corevia-desk-p1-ui-data.fa.md).

هدف اول: نسخهٔ پایدار و قابل‌تحویل **Full Desk مک و ویندوز** و **Customer Support مک و ویندوز**.
هیچ‌کدام تا وقتی نصب/اجرا، تمام صفحه‌ها و کنترل‌های UI، عمل واقعی متصل به هر کنترل،
داده/نشست، خطا/خالی/بارگذاری، RTL/LTR و Responsive روی سورس فعلی تست و Evidence
نشده‌اند آماده اعلام نمی‌شوند. اتصال Full Desk با Peer ID و مسیر ID/Relay معیار
اصلی است؛ IP، RDP یا هم‌شبکه‌بودن جایگزین نیست.
روی همهٔ پلتفرم‌های دسکتاپیِ پشتیبانی‌شده، کلیک دوباره باید پنجرهٔ همان پروفایل را
فعال/بازگردانی کند و پردازهٔ اصلی دوم نسازد؛ دو پروفایل می‌توانند کنار هم اجرا
شوند. Customer Support ثابت و موبایل‌اندازه است و Full Desk عریض و قابل‌تغییر می‌ماند.

تصمیم مالک محصول: گواهی امضای مک شرط ادامه نیست. رفتار مشترکی که مجوز مک جلوی آزمون مستقیمش را می‌گیرد با شاهد معادل Windows/سیستم‌عامل دیگر قابل‌پذیرش است، اما باید به‌عنوان شاهد جایگزین مشخص شود؛ قابلیت‌های بی‌نیاز از امضا روی خود مک همچنان باید آزموده شوند.

آخرین آزمون زنده: Full Desk فعلی به باندل قدیمیِ Customer Support روی همین مک با Peer ID و `/r` وصل شد؛ Relay رمزگذاری‌شده و تصویر واقعی بیش از ۴۰ ثانیه پایدار بود و نشست فقط‌تصویر بسته شد. این فقط شاهد کارکرد مسیر و باندل مجازشده است، نه قبولی نسخهٔ فعلی Customer Support: باندل تازه هنوز capture را `false` می‌خواند. macOS پس از Quit & Reopen ابتدا باندل قدیمی را بالا آورد؛ اجرای دستی باندل تازه هم مشکل را رفع نکرد. باندل‌های ad-hoc CDHash متفاوت دارند و هویت معتبر امضا نصب نیست؛ این سرنخ محتمل است، نه علت قطعی و هیچ TCC bypass/reset انجام نشد. Full Desk یک‌بار وضعیت انتظار کهنه نشان داد و با «بازکردن نشست» به Online/Relay/رمزشده تازه شد. `/` و Personal حدود ۴۰ GiB آزاد دارند؛ `.work` حدود ۱۵ GiB است و Docker دست‌نخورده ماند. تصویر نسخهٔ فعلی، Windows مستقل، ورودی و همهٔ ابزارهای زنده هنوز گیت بازند.

تازه‌ترین ادامه — ۲۰۲۶-۱۰-۰۸: روی باندل قدیمیِ مجازشده، اجازهٔ کیبورد/ماوس برای یک نشست پذیرفته شد اما هیچ ورودی‌ای ارسال نشد. نمایشگر مقصد محتوای شخصی مرورگر را نشان داد؛ تست فوراً متوقف، ورودی خاموش و نشست بسته شد. هیچ Toolbox/MCP اجرا نشد. باندل جاری Customer Support به تنظیمات macOS افزوده و روشن شد؛ پس از درخواست مجوز از خود آن، راه‌اندازی مجدد عادی و تأیید پردازهٔ جاری، ضبط تصویر هنوز غیرفعال بود. Full Desk Active=0 است. Devices برای Peer مکِ چندباراستفاده‌شده یک ردیف دارد (آخرین اتصال ۷ اکتبر، ۲:۰۷ بعدازظهر) و یک دستگاه قدیمی Windows جداست؛ Sessions نشست پایان‌یافته و تلاش آزمایشی ناموفق جداگانه را دارد. TCPهای 3389/5985 پاسخ دادند و 5986/22 نه؛ گواهی خودامضای RDP مورداعتماد قرار نگرفت. اجراگر Bridge، HTTP Basic و خاموش‌کردن بررسی گواهی را اجباری می‌کند و فقط آزمون شبکه است، نه نصب/آزمون تعاملی اپ؛ اجرا نشد. در بررسی شبکه‌ای اولیه فرمانی به میزبان دور ارسال نشد؛ بعدتر WinRM فقط‌خواندنی موفق شد، بدون تغییر روی سرور. Readback آن زمان `/` و Personal هرکدام حدود ۳۷ GiB آزاد داشتند؛ Docker دست‌نخورده ماند. [شواهد مسیر امن Windows](../evidence/corevia-desk-windows-safe-access-20261008.fa.md) و [Readback تازهٔ WinRM](../evidence/corevia-desk-windows-winrm-readback-20261008.fa.md).
بازآزمایی خودکار در ادامهٔ جاری: Web ‏۱۵/۱۵، MCP ‏۲۷/۲۷، Release ‏۱۰/۱۰ و lintهای Atomic/Profile پاس شدند؛ ورودی بومی و Toolbox زنده هنوز بازند.
Readback تازهٔ دیسک: علاوه بر پاک‌سازی ۵٫۵ GiB کش Cargo، حدود ۷۷ MiB کش قدیمی فهرست افزونه‌های Codex پاک شد. اکنون `/` و Personal هرکدام حدود ۳۸ GiB آزاد دارند؛ Docker، باندل‌های فعال و سورس حفظ شدند.
ممیزی رسمی مستندات Standards نیز روی snapshot `f663ba2` با Validate و Plan پاس شد: ۵۳ فایل، بدون finding؛ پیوندهای موقت حذف شدند.
آخرین refها در ۲۰۲۶-۱۰-۰۸: Standards برابر `f663ba2`، Bridge برابر `7bb1645` و Knowledge شاخهٔ `develop` برابر `e605a19` و `main` برابر `539b3e9`. Source Audit Bridge فقط روی snapshot جداگانه پس از به‌روزرسانی رسمی Catalog سبز است؛ این تغییر یک‌فایلی بالا‌دست منتشر نشده. سه ref دوردست Desk از قبل در `248afe1b` یکسان‌اند؛ working tree محلی کثیف و Push‌نشده است.

ادامهٔ ۲۰۲۶-۱۰-۰۸: Generate تولیدی Nuxt برای هر دو پروفایل، تست‌های Web ‏۱۵/۱۵، MCP ‏۲۷/۲۷، Release ‏۱۰/۱۰، lintهای Atomic/Profile/Scope و `git diff --check` پاس شدند؛ ممیزی رسمی Standards نیز ۵۳ فایل را در Validate/Plan بدون finding تأیید کرد. درخواست Peer ID با `/r` به Customer Support جاری رسید؛ درخواست رد شد و هیچ تصویر یا ورودی‌ای منتقل نشد، Active=0 ماند. اگر آزمون تکرار شود، ابتدا تأیید همان لحظه لازم است؛ سپس مسیر امن Windows و ماتریس ابزارها روی میزکار آزمایشی جدا. فضای آزاد فعلی حدود ۴۲ GiB است.

در همین ادامه، Apply آزمایشی Standards دو فایل اسپک را ناقص بازنویسی کرد؛ محتوای پذیرش بازسازی و با Parser YAML و Validator رسمی تأیید شد. Plan هنوز Update هر دو فایل را پیشنهاد می‌دهد؛ Apply تکرار نشد و خود الگو باید اصلاح شود. معیارهای اصلی محصول همچنان بازند.

درخواست آزمایشی تصویر تا دریافت تأیید همان لحظه باز نماند: رد شد، هیچ دادهٔ صفحه/ورودی منتقل نشد و Readback نشست‌ها Active=0 را نشان داد. فضای آزاد هر دو volume حدود ۴۲ GiB است.

ماتریس Responsive پیش‌نمایش وب در ۲۰۲۶-۱۰-۰۷ برای شش مسیر، پنج اندازه و دو جهت
۶۰/۶۰ پاس شد. ظاهر پیش‌نمایش Customer Support تیره/RTL نیز در ۱۲۸۰×۷۲۰ دوباره
بررسی شد و با سبک قبلی هماهنگ است، اما مرورگر پل بومی ندارد. رفتار بومی چهار نسخه
و تست عملکرد کنترل‌ها همچنان باز است.

آزمون تازهٔ باندل مک در همین روز مسیر Peer ID و `/r` را به Relay رمزگذاری‌شده و
Connected رساند، اما فریم واقعی نداد؛ Customer Support هنوز اجازهٔ تصویر را false
می‌خواند. درخواست رسمی مجوز داخل برنامه و بستن/بازکردن همان باندل هم نتیجه را عوض
نکرد؛ علت دقیق اختلاف امضا اثبات نشده است. طبق تصمیم مالک، این مورد کار را متوقف
نمی‌کند و رفتار مشترک باید روی Windows به‌صورت جایگزین آزموده شود. نشست بسته شد و
Active=0 شد. Devices یک ردیف با زمان ۹:۰۹ و Sessions یک کارت آخرین وضعیت برای همان
Peer ID نشان داد. اتصال مستقل Windows و تک‌تک ابزارها همچنان بازند.

تست مستقیم بومیِ مک که امضا نمی‌خواست: Customer Support زبان RTL/LTR و تم روشن/تاریک را تغییر داد و به حالت اولیه برگشت؛ Full Desk پیمایش چهار صفحه و تغییر/بازگردانی پنج گزینهٔ نمایش را پاس کرد. شناسهٔ خالی پیش از نشست رد شد؛ تلاش با شناسهٔ آزمایشی سفارشی شکست خورد، فقط در تاریخچهٔ Sessions ثبت شد و Device/Home recent نساخت. ابزارهای واقعی در نشست زنده و آزمون مستقل Windows هنوز بازند.

در MCP نیز مسیر خطای اجرای ناقص اصلاح و با آزمون رگرسیون پوشش داده شد: مجازشدن
درخواست دیگر به‌تنهایی موفقیت عملیات محسوب نمی‌شود. ارسال پیام تأیید جداگانه دارد
و متن پیام در بازبینی نشان داده می‌شود. آزمون MCP فعلی ۲۶/۲۶ پاس است؛ قرارداد
مقصد اختیاری ابزار هدایت پورت هم در MCP و Rust هم‌تراز شد و ورودی خراب fail-closed
است. تونل زنده هنوز آزموده نشده؛ این تغییر
اجراگرهای غایب Camera/Printer/Service یا تست واقعی Toolbox را نمی‌بندد.
UI نیز خطای اجرای ناقص را اعلام می‌کند و حالت را جلوتر از Readback تغییر نمی‌دهد؛
وب ۱۴/۱۴ و Build تولیدی پاس‌اند. ماتریس زندهٔ ابزارها همچنان باز است.

Readback واقعی MCP روی Bundleهای `1.5.36-local.20261007` مک نیز انجام شد:
کاتالوگ‌ها ۲۵/۹ ابزار و پنج Read امن در هر پروفایل پاسخ `accepted` دادند.
این فقط اتصال Readها به پل Rust را ثابت می‌کند؛ Mutationهای تأییدشده و ابزارهای
نشست هنوز بازند و سه اجراگر Camera/Printer/Service وجود ندارد.

آزمون واحد Rust روی macOS arm64 با vcpkg پین‌شدهٔ خود مخزن برای Full Desk
۱۳۱/۱۳۱ و برای Customer Support نیز ۱۳۱/۱۳۱ پاس شد. هدف موقت ۲٫۲ GiB روی
Personal ساخته و پس از آزمون پاک شد. این فقط منطق واحد را می‌سنجد؛ ورودی زندهٔ
دو میزبان، UI ویندوز و تک‌تک Toolboxها هنوز بازند.

در ادامهٔ ۲۰۲۶-۱۰-۰۷ هر دو Bundle مک با نسخهٔ `1.5.36-local.20261007` ساخته و
Smoke شدند و هر دو در UI بومی باز شدند؛ برای هر پروفایل یک پردازه باقی ماند.
Customer Support عنوان متمایز و پنجرهٔ ثابت/موبایل‌اندازه دارد. مجوزهای هم‌نام در
تنظیمات روشن‌اند، اما باندل تازه هنوز تصویر را false می‌خواند؛ روی مک هم هویت معتبر
امضا وجود ندارد؛ امضای دقیق نسخهٔ تازه با باندل قبلی متفاوت است و احتمال mismatch
مجوز را بالا می‌برد، اما علت قطعی نیست. نسخه‌ها ad-hoc هستند؛ امضای انتشار،
فریم واقعی نسخهٔ جاری، ورودی روی دو میزبان و UI بومی کامل هنوز بازند.

Readback تازهٔ ۲۰۲۶-۱۰-۰۶: Full Desk مک تصویر پایدار و reconnect را در خوداتصال
نشان داد و MCP نشست رمزگذاری‌شدهٔ ID/Relay را ساخت و بست؛ دو میزبان مستقل هنوز
پاس نیست. Customer Support ویندوز از همان سورس Build و Smoke شد و WinRM با
رمزنگاری پیام NTLM کار کرد. درخواست واقعی `/r` از مک به Relay رسید اما رمزنگاری
نشد و به Customer Support نرسید؛ ویندوز نشست گرافیکی قطع و بدون نمایشگر داشت.
Full Desk ویندوز از قبل روی همان میزبان و با همان Peer ID فعال است، پس مقصد ایزوله
نیست. هشدار گواهی RDP لغو شد، نه دور زده. جزئیات در [Evidence ویندوز](../evidence/corevia-desk-windows-customer-support-20261006.fa.md)
است. بیلد Windows امضا و Installer ندارد. شاخه‌ها پیش‌تر همگی روی
`248afe1b69a73efe171c12d2714cd42a942fb04f` خوانده شدند؛ تغییر محلی منتشر نشده است.

| گیت | وضعیت واقعی در ۲۰۲۶-۱۰-۰۶ | قدم بعدی |
| --- | --- | --- |
| شاخه‌های Desk | `248afe1b69a73efe171c12d2714cd42a942fb04f` روی شاخهٔ کاری، `develop` و `main`؛ tag `v1.5.36` هم همین commit است | sync تکراری نکن مگر Readback drift نشان دهد |
| Source Audit Bridge | `ok: true` روی snapshot تمیز آخرین `develop` با SHA `5952da0`؛ Catalog Write لازم نشد | پیش از هر عملیات بیرونی Audit را حفظ کن |
| Standards و Knowledge | snapshot تمیز Standards روی `origin/develop` برابر `bdfe0b9`؛ Knowledge: `main` پیش‌فرض `539b3e9` و `develop` جدیدتر `e6a2a01`؛ checkoutهای اصلی کثیف حفظ شدند | برای Standards/Knowledge فقط snapshot تمیز به‌روز استفاده کن |
| اتصال، تصویر و ورودی؛ سپس MCP/ابزارها | باندل قدیمیِ مجازشده تصویر واقعی را از Relay بیش از ۴۰ ثانیه نشان داد؛ ابزارها خاموش بودند. باندل جاری هنوز تصویر نمی‌دهد؛ Full Desk وضعیت انتظار را با «بازکردن نشست» تازه کرد. ورودی و Toolbox بازند | با Windows مستقل، همان مسیر Peer ID/Relay، تصویر، پایداری و گذار وضعیت را بسنج؛ سپس ورودی و تک‌تک ابزارها/MCP را با Readback جداگانه تست کن. قبولی باندل قدیمی را به جاری تعمیم نده |
| ضبط تصویر مک | باندل جاری بعد از اجرای دستی هم `false` می‌خواند؛ macOS در Quit & Reopen ابتدا مسیر باندل قدیمی را باز کرد. هر دو باندل ad-hoc با CDHash متفاوت‌اند و هیچ هویت معتبر امضا نیست؛ mismatch مظنون است، نه علت اثبات‌شده | مجوز را دور نزن یا reset نکن. اگر مسیر رسمی امضای پایدار در دسترس نیست، شاهد معادل Windows را صریحاً جایگزین ثبت کن و قابلیت‌های بی‌نیاز از امضا را روی مک ادامه بده |
| Customer Support ویندوز | بیلد قبلی `1.5.36-test.248afe1b6`، Smoke، تک‌نمونه و Read امن MCP پاس؛ Authenticode و Installer ندارد. UI زندهٔ ویندوز تأیید نشده. اجراگر رسمی Bridge هنوز آزمایشی است و HTTP Basic را با بررسی گواهی خاموش اجباری می‌کند؛ اجرا نشد. [شواهد مسیر امن](../evidence/corevia-desk-windows-safe-access-20261008.fa.md) | مسیر رسمی امن لازم است؛ سپس بیلد جاری را روی دسکتاپ فعال و جدا تست کن: تصویر `/r`، پایداری/reconnect، ورودی و تک‌تک ابزارها |
| CI/Nexus | Pattern تازه به Values غایب ارجاع می‌دهد و `gitlab-pipeline-current` در Bridge نیست | فقط از مسیر رسمی Standards → Bridge → `connectcore` شکاف را حل کن؛ اجرای دستی ممنوع |
| Linux/Docker | اولویت پایین‌تر از چهار نسخهٔ Desktop. فایل اصلی Docker از قبل با پیوند روی Personal است (۴۲ GiB آزاد)؛ ایمیج‌ها، کانتینرها، کش Build و لاگ کانتینر حفظ شده‌اند. حدود ۱۱ MiB لاگ تشخیصی جداگانهٔ Docker Desktop روی Data مانده است. با وجود نشان «Engine running»، فهرست ایمیج و API موتور پاسخ نمی‌دهند. | پس از سالم‌شدن موتور، مسیر Personal را از تنظیم Docker → Resources → Advanced ثبت/تأیید کن و سپس Linux/Docker را تست کن. تا وقتی موتور پاسخ نمی‌دهد، فایل را دستی جابه‌جا یا Docker را Restart/Purge نکن. |
| VS Code | آینده و خارج از تحویل | فعلاً کاری نکن |

### آخرین Readback اجرایی و دیسک — ۲۰۲۶-۱۰-۰۶

- مسئلهٔ تصویر در باندل تازهٔ Full Desk مک/arm64 بومی Verify شد: دو تلاش واقعیِ ردشده برای یک Peer ID به یک کارت تبدیل شدند؛ زمان تلاش دوم، برچسب کامل «Failed»، شمارندهٔ یک و صفر نشست فعال Readback شد. این فقط یک ترکیب و History ناموفق را پوشش می‌دهد؛ P1 باز است. target موقت ۲٫۱ GiB پاک و باندل ۲۴ MB نگه داشته شد؛ `/` و Personal هرکدام حدود ۶۲ GiB آزاد دارند.

- CI/Nexus: Validate/Plan رسمی Source Catalog، تصمیم capability و governance روی Bridge `5952da0` و Standards `bdfe0b9` PASS است؛ تصمیم Extend. فرمان موجود فقط آخرین Pipeline ناموفق را retry می‌کند و برای commit دقیق جاری/Artifactها کافی نیست. Pattern محصول `draft` است، Values و عملیات `connectcore platform gitlab-pipeline-current` موجود نیستند. هیچ Pipeline/Publish اجرا نشد؛ تغییرات Desk هم ثبت‌نشده‌اند و روی Remote نیستند.
- QA مرورگر روی سورس جاری ۶ مسیر Full Desk/Customer Support را در ۵ عرض (۳۲۰ تا ۱۴۴۰) و هر دو RTL/LTR سنجید: ۶۰/۶۰ بدون سرریز و خطای Console. اعتبارسنجی اتصال/فرم خالی، فیلترهای Devices، جست‌وجوی بی‌نتیجه و Cancel داده نساختند. تنظیمات واقعی بدون Native Host آزمون‌پذیر نبودند. Nuxt dev چند خطای موقت `#app-manifest` در HMR نوشت؛ مسیرها رندر شدند. دیسک اکنون ۶۷ GiB آزاد دارد؛ Docker image/cache حفظ است.
- تازه‌ترین اجرا: هر دو Bundle مک/arm64 `1.5.36-mcp-elicitation-test8` Smoke دقیق PASS؛ Rust ۱۲۵/۱۲۵ هر پروفایل، MCP Node ۲۲/۲۲ و Release ۱۰/۱۰. MCP stdio به‌صورت شبیه‌سازی‌شده تأیید میزبان و بررسی helper Rust را برای هر دو پروفایل پاس کرد؛ کلاینت واقعی هنوز آزموده نشده است.
- UI فعال `test6` قبلاً فقط‌خواندنی بازبینی شد؛ تست اول درخواست را پس از بازگشت به Home نشان داد و رد کرد. تست دوم معیار تازه‌سازی خودکار را قطعی نکرد. Bundleهای بومی Customer Support `test10` و Full Desk `test11` Smoke شدند؛ دیالوگ اتصال تکراری/باریک در `test11` اصلاح و به‌صورت بومی تصویری/دسترس‌پذیری تأیید شد. Peer ID خالی نشست نساخت و هیچ درخواست شبکه‌ای در این Readback نبود. کلاینت واقعی MCP SDK روی helperهای بومی هر دو پروفایل کاتالوگ ۲۵/۹ و پنج/چهار Read امن را پاس کرد؛ Elicitation واقعی و عملیات حساس باز است. UI زندهٔ درخواست هنگام visible بودن همچنان باز است؛ پیش از ارسال بعدی، همان عمل انتقال Peer ID به مقصد را تأیید کن. Documentation Audit رسمی روی ۴۹ فایل بدون Finding PASS شد. پس از حذف target موقت Cargo، فضای آزاد حدود ۵۳ GiB روی `/` و Personal بود؛ Bundle ۲۴ MB ماند و Docker image/cache دست‌نخورده است.
- QA مرورگری سورس تازه Home، Devices، Sessions، Settings و هر دو صفحهٔ Customer Support را در ۳۹۰×۸۴۴، ۷۶۸×۱۰۲۴ و ۱۲۸۰×۷۲۰ بدون سرریز افقی PASS کرد. دکمهٔ اتصال سریع Home حالا کادر Peer ID را متمرکز می‌کند؛ فرم‌های خالی/ناقص رکورد نساختند. Profile/Atomic lint و Build تولیدی پاس شدند. UI بومی `test8`، LTR و هر عمل واقعی هنوز بازند.

- سابقهٔ `test7` پیش از Elicitation: Bundleها و تست‌های وقت آن PASS شدند؛ عملیات حساس به‌دلیل نبود اثبات میزبان Deny می‌شد. این وضعیت با ادامهٔ تازهٔ بالای همین بخش جایگزین شده است. Probe outbound ساختگی پیشین را با ID ساختگی تکرار نکن.
- Readback تازهٔ `test5`: Full Desk و Customer Support مک/arm64 با نسخهٔ `1.5.36-single-instance-test5` Smoke دقیق انتشار را پاس کردند و About همان نسخه را نشان داد. Rust هر دو پروفایل ۱۲۰/۱۲۰ PASS شد. تغییر تم/زبان آغازین و راهنمای مجوزها در Native UI درست‌اند؛ Customer Support زبان انگلیسی را پس از Restart حفظ کرد و سپس فارسی اولیه بازگردانده شد. Finder برای هر Bundle فقط یک پردازه نگه داشت. فرم خالی Peer ID فقط اعتبارسنجی داد؛ فهرست/فیلتر دستگاه و حالت خالی Sessions درست بودند. اتصال واقعی و ماتریس کامل هنوز باز است. فضای فعلی Data حدود ۳٫۲–۳٫۴ GiB و Personal حدود ۶۱–۶۲ GiB آزاد است.
- آزمون تکمیلی روی همین Bundleها: درخواست Peer ID با `/r` از Full Desk به Customer Support همین دستگاه، مسیر «Relayed via ID/Relay» را نشان داد اما به UI مقصد نرسید؛ مبدأ در انتظار ماند و پس از بستن اتصال ناموفق ثبت شد. `read-sessions` بومی MCP برای هر دو پروفایل Accepted و خالی بود. این تست اتصال کامل را تأیید نمی‌کند.
- Full Desk و Customer Support مک با نسخهٔ آزمایشی `1.5.36-single-instance-test4` ساخته و Smoke دقیق نسخه را پاس کردند؛ About برنامه نیز نسخهٔ یکسان با بسته را نشان داد. پس از کوچک‌کردن و بازکردن دوباره از Finder، هر دو همان پنجره را برگرداندند؛ دوبارکلیک دوباره روی هر Bundle نیز پردازهٔ دوم نساخت.
- در تست بومی محدود، Full Desk صفحه‌های Home، Sessions، Devices و تنظیمات را باز کرد؛ راهنما Peer ID از ID/Relay را اصلی، `/r` را برای اجبار Relay و IP را فرعی می‌داند. فرم خالی نشست نساخت، فیلتر/جست‌وجوی بی‌نتیجه درست بود و لغو افزودن دستگاه داده را تغییر نداد. Customer Support حالت بدون درخواست و محدودیت مجوزهای پروفایل را درست نشان داد. این تست جای ماتریس کامل را نمی‌گیرد.
- در Readback تاریخی `test4` ناهماهنگی Startup کشف شد: Home پیش‌فرض فارسی/تاریک بود و Settings انگلیسی/روشن ذخیره‌شده را داشت. Source اصلاح شد؛ Readback Native بعدی روی Bundle `test5` در بند نخست ثبت شده است.
- تست cross-target ماژول تک‌نمونه و تست‌هایش در Windows/MSVC سبز است؛ این جای Build/اجرای بومی Windows را نمی‌گیرد. Windows کامل، Linux بومی، UI/دادهٔ کامل، درخواست ورودی واقعی، MCP سورس فعلی و CI/Nexus همچنان گیت‌های بازند.
- آخرین Readback دیسک: Data مک ۱٫۶ GiB آزاد/۹۹٪ پر؛ Personal ۶۶ GiB آزاد/۳۸٪ مصرف. Cargo خروجی ۹٫۸ GiB را پاک کرد، Go حدود ۳۳۰ MiB و npm حدود ۵۰ MiB آزاد کردند. NuGet پاک شد اما بخشی از cache کنار ابزارهای فعال .NET دوباره ساخته شد و پاک‌سازی تکرار نشد. Build وب روی Personal بود؛ Bundleهای تست حفظ شدند؛ Docker، Codex و کش‌های فعال Chrome/VS Code دست‌نخورده‌اند. بیلد حجیم روی Data اجرا نشود.

آخرین Fetch بعدی در Desk، Bridge، Standards و Knowledge تغییر refی نداشت. Remoteهای Desk روی SHA انتشار هستند؛ working tree جاری کثیف و ثبت‌نشده است و submodule کثیف قبلی نیز حفظ شد؛ switch، stage، commit یا push نشد. Bridge روی `develop` آخر است؛ Standards snapshot روی آخرین `develop` است، اما checkout اصلی متعارض حفظ شد؛ Knowledge snapshot برابر `origin/main` است و checkout اصلی کثیف حفظ شد. Documentation Audit پس از به‌روزرسانی روی ۴۹ فایل و Source Audit رسمی Bridge دوباره PASS شدند. `corevia-desk-ui` Git repo نیست.

Readback تازه در ۲۰۲۶-۱۰-۰۴: Volume دادهٔ macOS حدود ۱٫۵ GiB آزاد و ۹۹٪ مصرف است؛ روی Personal حدود ۵۶ GiB آزاد است. کش Cargo حذف نشد؛ به Personal منتقل شد و مسیر اصلی آن حفظ شد. فایل‌های موقت و خروجی Build بعدی باید روی Personal باشند. پوشهٔ امضای موقت Chrome که پردازهٔ زنده از آن استفاده می‌کند و کش به‌روزرسانی VS Code حذف نشدند؛ Docker image/cache، سورس و دادهٔ محصول هم دست‌نخورده‌اند. جزئیات و ماتریس تک‌نمونه در [Evidence تک‌نمونه](../evidence/corevia-desk-single-instance.fa.md) آمده است.

## Snapshot تاریخی تحویل — ۲۰۲۶-۰۹-۱۸ (مرجع جاری نیست)

بخش بعدی برای تاریخچه حفظ شده است. برای وضعیت و اولویت کنونی، جدول بالا و
`docs/evidence/current-status.fa.md` مرجع هستند. هیچ فازی نباید با حدس یا صرفاً
با آماده‌بودن قرارداد و گیت، کامل اعلام شود.

### آخرین Readback ادامه — ۲۰۲۶-۰۹-۱۸

- commit `939f46c0594cf7a251acae34ca862f1d96dfb338` و tag immutable `v1.5.32` منتشر شده‌اند؛ Sync رسمی Bridge شاخه‌های `codex/desk-phases-8-12` → `develop` → `main` را به‌ترتیب و فقط با Fast-forward روی همین SHA انجام داده است.
- pipeline `3536` همهٔ Buildها و انتشار package را تمام کرده است؛ فقط verify manifest داکر با job `24630` به‌دلیل پاسخ ۴۰۱ از Nexus شکست خورد، بعد از آن‌که Windows Full Desk با job `24627` روی Runner بومی اصلاح‌شده پاس شد.
- Evidence Runner پشتیبانی long-path سیستم، Rust/Cargo، دسترسی API، سرویس Running و heartbeatهای پذیرفته‌شده را تأیید می‌کند؛ backlog قدیمی Windows فقط با cancel رسمی و graceful پاک شد.
- P4 تا اجرای tag immutable تازه با verify موفق داکر و ثبت Readback دقیق Image `linux/amd64` و packageها در Nexus باز است؛ Image واقعی `1.5.32` از مسیر رسمی Bridge Readback موفق دارد. Install جانبی/UI تصویری Windows در P1.5 و مثبت‌بودن Secret در P3 نیز باز هستند.

### هدف اصلی و اولین معیار موفقیت

اول باید این سناریو واقعاً کار کند:

1. **Full Desk مک** بدون واسطه‌ی MCP به‌عنوان برنامه‌ی اصلی اجرا شود.
2. **Full Desk ویندوز** بدون واسطه‌ی MCP به‌عنوان برنامه‌ی اصلی اجرا شود.
3. هر دو برنامه با تنظیمات پیش‌فرض ID Server و Relay Server، از طریق **شناسه‌ی
   RustDesk** به هم وصل شوند؛ هم‌شبکه‌بودن یا اتصال مستقیم با IP معیار موفقیت
   نیست.
4. درخواست اتصال، تأیید، فعال‌شدن نشست، تصویر زنده، کنترل ماوس و صفحه‌کلید،
   بستن و اتصال دوباره با موفقیت ثبت شود.
5. در هر دو سمت، خود محصول روشن و صاحب نشست باشد؛ MCP فقط ابزار بررسی و کنترل
   پس از پاس‌شدن این گیت است و جای محصول اصلی را نمی‌گیرد.

تا وقتی این پنج مورد با Evidence قابل تکرار پاس نشده‌اند، تحویل اصلی و CD-12
کامل اعلام نمی‌شود.

### ترتیب قفل‌شده‌ی تحویل

| اولویت | کار | وضعیت مرجع |
| --- | --- | --- |
| P0 | Full Desk مک↔ویندوز، اتصال با شناسه/Relay و کنترل کامل نشست | باز/جزئی؛ `/r` روی همان مک تصویر زنده و reconnect را نشان داد؛ وضعیت Connected بومی درست است. ورودی به TextEdit نرسید. Peer ID عادی در خوداتصال ورودی نساخت؛ تست دو میزبان و Windows لازم است |
| P1 | امکانات پایه با قالب Nuxt: UI/داده، نشست‌ها، تنظیمات، ذخیره، Clipboard، Toolbox، نشست هم‌زمان و RTL/LTR | باز؛ UI پایهٔ هر دو باندل مک خوانده شد؛ جست‌وجو/فیلتر خالی، اعتبارسنجی Add و Cancel بدون ثبت داده کار کردند. Web 11/11 و Rust Full Desk 131/131؛ ابزارهای زنده، ماندگاری کامل و هر چهار نسخه هنوز بازند |
| P1.5 | نصب و اجرای Customer Support و Full Desk مک و ویندوز با ACL جدا و تک‌نمونه | باز؛ اجرای بومی و تک‌نمونهٔ مک برای هر دو پروفایل بررسی شده؛ نصب جانبی/UI تصویری و تک‌نمونهٔ Windows باز است |
| P2 | همهٔ ابزارهای MCP با همان ACL، تأیید، ثبت رخداد و رد امن | باز؛ MCP ‏۲۷/۲۷؛ قرارداد `remoteHost` اختیاری و Boolean اختیاری روشن/خاموش تماس صوتی با رد ورودی خراب پاس شد؛ روی Bundle جاری مک کاتالوگ ۲۵/۹ و پنج Read امن در هر پروفایل از پل Rust `accepted` شدند؛ Camera/Printer/Service، Mutation تعاملی، Audit و اجرای زندهٔ ابزارها هنوز بازند |
| P3 | اعلان‌های Success/Warning/Danger/Info و نمایش کنترل‌شدهٔ Secret | باز؛ آزمون مثبت Secret و بررسی تصویری چهار نسخه باقی است |
| P4 | CI و انتشار تغییرناپذیر در Nexus برای commit دقیق جاری و Readback همهٔ خروجی‌ها | باز؛ Pattern/Values و executor رسمی اجرای pipeline دقیق commit در Bridge حاضر نیست؛ اجرای دستی یا انتشار انجام نشود |
| P5 | Full Desk لینوکس و Docker لینوکس، با معماری درست سرور | اولویت پایین‌تر؛ پس از پاس‌شدن گیت‌های مک/ویندوز |
| P6 | افزونهٔ احتمالی VS Code | آینده و خارج از تحویل جاری |

## ادامهٔ تازه — ۲۰۲۶-۱۰-۰۶

- تاریخچهٔ نشست برای هر Peer ID نرمال‌شده یک کارت از آخرین وضعیت/زمان نشان می‌دهد؛ وضعیت ضربدری با برچسب متنیِ غیرکلیک‌پذیر عوض شد و شمارندهٔ اتصال نیز دستگاه تکراری را یکی می‌شمارد. رویداد خام حذف نمی‌شود.
- تست منطق تاریخچه ۴/۴ و lintها پاس؛ هر دو باندل مک Build/Smoke و UI تازه اجرا شد. تاریخچهٔ staging خالی بود، بنابراین کارت پُر هنوز Readback بومی نشده است.
- ممیزی دوزبانهٔ Standards و Validate/Plan مشخصات PASS شدند؛ Apply عمومی اجرا نشد.
- target موقت Cargo به‌اندازهٔ ۲٫۱ GiB پاک و Bundleهای staging حدود ۴۷ MB حفظ شدند؛ `/` و Personal هرکدام حدود ۶۱ GiB آزاد دارند. Docker دست‌نخورده ماند. CI/Nexus هنوز executor رسمیِ commit دقیق جاری را ندارد؛ Commit/Push انجام نشد.

### مرزهای غیرقابل‌تغییر

- مسیر فعال فقط **Rust + Nuxt داخل WebView بومی** است.
- Flutter و Dart و Sciter خارج از Scope فعال Build، Test، Installer و Release
  هستند و در ادامه به آن‌ها برنمی‌گردیم.
- اتصال موفق یعنی اتصال با شناسه از مسیر ID/Relay؛ تست IP یا هم‌شبکه‌بودن به‌تنهایی
  قابل قبول نیست.
- MCP یک Companion محلی و اختیاری با همان ACL است؛ فعلاً جدا اجرا می‌شود تا
  با Connection Manager محصول رقابت نکند، اما باید در نهایت با محصول یکپارچه و
  قابل‌کنترل باشد.
- کلیدها و Secretها فقط از تنظیمات Runtime یا CI خوانده می‌شوند و داخل سند،
  سورس، Log یا Artifact قرار نمی‌گیرند.

### وضعیت انتشار سورس و شاخه‌ها

- commit جاری Release روی شاخه‌ی `codex/desk-phases-8-12` برابر `939f46c0594cf7a251acae34ca862f1d96dfb338` است و Readback رسمی Sync روی همین SHA سبز است.
- `codex/desk-phases-8-12` به `develop` و سپس `develop` به `main` فقط با Fast-forward همگام شده‌اند و هر سه ref روی همین SHA هستند؛ tag immutable `v1.5.32` نیز Readback شده است.
- Plan رسمی Service Repo برای Push فقط `main` را هدف می‌گیرد؛ به‌دلیل خطر دورزدن ترتیب ارتقای شاخه‌ها Apply نشد. Pattern/Executor رسمی `gitlab-branch.publish` برای انتشار شاخه‌ی کاری با fast-forward-only و Readback commit اضافه و Plan شده است.
- Values رسمی `.corevia/operations/gitlab-branch-sync.values.yaml` ترتیب مجاز را ثبت می‌کند: شاخه‌ی کاری به `develop` و سپس `develop` به `main`، فقط با `fast-forward-only`.
- Source Audit رسمی Corevia Bridge بعد از اجرای مجاز `source catalog --write` سبز شد و Readback محافظت شاخه‌ها نیز تأیید شد؛ هیچ force-push یا تغییر دستی استفاده نشد.
- در ۲۰۲۶-۰۹-۱۰ Readback رسمی Runner مقصد image را `linux/amd64` تأیید کرد و imageهای immutable Node و Debian به Nexus رسیدند. Pipeline `3536` آن‌ها را بدون خطای image مصرف کرد و Artifactهای Linux و Docker را ساخت؛ فقط Windows Full Desk و Publish/Readback نهایی باقی است.

### وضعیت واقعی این لحظه

- در اجرای واقعی، Full Desk مک با Full Desk ویندوز از مسیر شناسه/ID و Relay وصل
  شد؛ Handshake و Login مقصد، تصویر زنده‌ی ۱۲۸۰×۸۰۰ و فریم‌های پیوسته ثبت شدند.
- کنترل واقعی ماوس با بازکردن Start و PowerShell ویندوز و بستن پنجره از مک پاس
  شد. نشست از رابط محصول بسته و با همان شناسه دوباره برقرار شد.
- باینری قبلی ویندوز Login را در صف نگه می‌داشت، چون قبل از وصل‌شدن به Connection
  Manager منتظر وضعیت پیش‌ورود می‌ماند. این مسیر در سورس اصلاح شده است تا ابتدا
  Listener بومی محصول را پیدا کند و فقط در نبود آن سراغ مسیر قدیمی برود.
- Build اصلاح‌شده‌ی Full Desk ویندوز با کد خروجی صفر ساخته و روی نصب ویندوز
  جایگزین شده است؛ رابط جدید Nuxt و دکمه‌ی تأیید نشست Pending داخل بسته‌ی واقعی
  قرار دارد و نسخه‌ی قبلی پشتیبان‌گیری شده است.
- UI بومی Nuxt روی ویندوز Load شده و مقصد با WinRM به Console منتقل شد تا ضبط
  صفحه پایدار باشد؛ تست محصول با RDP انجام نشد.
- گیت P0 بسته شده است. HTTP Audit/API روی پورت ۲۱۱۱۴ هنوز Timeout دارد، اما مسیر
  ID/Relay و کنترل نشست را متوقف نکرد.
- Evidence ماتریس P1 برای Responsive، RTL/LTR، Devices، Sessions، Settings،
  Persistence، Empty State و Customer Support UI در
  `docs/evidence/corevia-desk-p1-ui-data.fa.md` ثبت شده است. Bridge اکنون
  تفاوت Direct، Relayed via ID/Relay و Negotiating را به خود UI می‌رساند.
- ماتریس MCP، Notification و Secret eye نیز در کد و Build بررسی شده‌اند؛ Smoke مرورگر داخلی هر دو پروفایل در سه اندازه، RTL/LTR و سلامت Console نیز سبز است و Preview بدون Bridge نشست/اعلان جعلی نساخت. تست زندهٔ تازه، Online/Relay، تصویر Windows، تأیید ورودی و دو کنترل نشانگر/فقط‌مشاهده را پاس کرد؛ نتیجهٔ کامل Native/GUI و انتشار بیرونی هنوز به گیت‌های بعدی وابسته است.
- Customer Support Windows روی Runner دارای C/Windows SDK native build شد؛ PE نسخهٔ ۱.۵.۰، archive، manifest و smoke رسمی PASS هستند. Native MCP headless نیز readbackهای پایه و denyهای Secret/SystemCommand را PASS کرد؛ اجرای portable با `--no-server` process دقیق bundle را در Session 2 نشان داد، اما UI Automation خروجی قابل اتکا نداد، پس Install جانبی و UI تصویری هنوز باز است.
- مسیرهای Release برای Windows با `fileURLToPath` و shell درست `npm.cmd` اصلاح و تست شدند؛ متن Devices و Dialog اتصال نیز Peer ID مقصد را مسیر اصلی و IP را مسیر ثانویه معرفی می‌کند.
- Audit بسته‌بندی یک Bundle قدیمی Customer Support مک را که metadata محدود اما native binary آن Full Desk بود رد کرد. اکنون Build Release از Targetهای Cargo، هویت application و namespace بومی جدا بر اساس profile/platform/architecture استفاده می‌کند و `COREVIA_SKIP_CARGO_BUILD` را رد می‌کند؛ Bundle Customer Support مک با vcpkg پروژه دوباره ساخته شد و دو Peer ID مستقل با دو نشست هم‌زمان Active، Relay و امن Readback شدند. Install، UI تصویری ویندوز و Readback بیرونی هنوز باز هستند.
- انتقال واقعی محتوای Clipboard دوطرفه با marker زنده Readback شد و PASS است؛ گیت دو نشست هم‌زمان Active با دو مقصد مستقل نیز بسته شد. Toggle و Guard تکرار به‌تنهایی معیار نبودند و Evidence مستقل برای هر دو مقصد ثبت شد.
- Dialog اتصال پسوند صریح RustDesk یعنی `/r` را به‌عنوان انتخاب اختیاری Relay می‌پذیرد؛ Bridge آن را پیش از اعتبارسنجی نرمال می‌کند و Peer ID هویت اصلی می‌ماند. دو درخواست مستقل `/r` تأیید و با `direct:false` و `secured:true` بیش از ۲۰ ثانیه Active ماندند و تمیز بسته شدند؛ Readback تصویری خود Selector به‌دلیل قفل UI مک هنوز باز است.
- تنظیمات پیش‌فرض سرور در Runtime هر دو سمت اعمال شده‌اند؛ مقدار حساس Key در
  سند یا گزارش تکرار نمی‌شود.

## نقطه‌ی پایان / تعریف تکمیل

- یک هسته‌ی موجود Rust منبع اصلی دسترسی سیستم و رفتار دسکتاپ راه دور باقی می‌ماند.
- یک ACL اجباری و لایه‌ی مشترک دستورات به رابط فعال Nuxt/WebView و MCP سرویس می‌دهد.
- لایه‌ی View مبتنی بر Atomic Design داخل `corevia-desk` و از روی منبع خام `corevia-desk-ui` ساخته می‌شود.
- Customer Support و Full Desk برای هر محیط، پروفایل انتشار و بسته‌ی جدا در Nexus دارند.
- استفاده از Corevia Installer اختیاری است و فقط می‌تواند Customer Support را نمایش دهد.
- Corevia Bridge فقط مالک نصب عملیاتی تأییدشده، سلامت، به‌روزرسانی، Rollback و تنظیمات Runtime است.
- موبایل تا زمان اثبات مسیر دسکتاپ‌محور تعویق دارد.

## وضعیت فعلی

- CD-1 آماده‌سازی استانداردهای Corevia: کامل.
- CD-2 مرزبندی و تصمیم معماری: کامل.
- CD-3 ACL مشترک Rust و مرز دستورات: کامل.
- CD-4 لایه‌ی View مبتنی بر Atomic Design در Nuxt: کامل.
- CD-5 پل Nuxt به Rust: پیاده‌سازی‌شده؛ اتصال بومی آن در CD-5A انجام شده است.
- CD-5A Host بومی WebView و اتصال Runtime: در macOS arm64 پیاده‌سازی و در Smoke واقعی محصول استفاده شد؛ بررسی Runner و Release باقی است.
- CD-6 لایه‌ی کنترل MCP: در محیط محلی کامل؛ کاتالوگ ۲۵/۹ و Native MCP Readback برای Mutation، Approval، Audit، نشست ناموجودِ بدون اجرا و Denyهای پروفایلی/حساس در هر دو پروفایل پاس شده است؛ انتشار خارجی در CD-7/P4 باقی است.
- CD-7 مسیر CI/CD و انتشار Nexus: در pipeline `3536` قراردادها، هر دو Artifact مک، Linux، بسته‌بندی Docker، هر دو Build ویندوز و انتشار package روی Runner بومی اصلاح‌شده PASS شده‌اند؛ فقط verify manifest داکر با ۴۰۱ شکست خورد. Image واقعی `1.5.32` از Readback رسمی Bridge برای `linux/amd64` پاس شد و اصلاح Bearer در tag immutable بعدی باید CI و Readback کامل را ببندد.
- job Docker فعلی Image را مستقیم به Nexus اعلام‌شده Push می‌کند و فقط metadata کوچک نگه می‌دارد؛ Readback دقیق Image محصول بعد از مرحلهٔ Publish تکرار می‌شود و هیچ جایگزینی حدسی یا تغییر معماری مجاز نیست.
- CD-8 محصول Customer Support: UI، ACL، MCP، macOS، Windows archive/manifest/smoke، process launch، namespaceهای جدا، Readback هم‌زمان Active و Runner رسمی بررسی شده‌اند و Artifact محافظت‌شدهٔ Windows نیز در pipeline جاری تولید شده؛ Install جانبی و UI تصویری هنوز باز است.
- CD-9 محصول Full Desk: در محیط محلی پیاده‌سازی شده؛ Artifactهای macOS/Linux/Docker/Windows در `3536` سبز هستند، انتشار Docker انجام شده و فقط روش verify احراز هویت شکست خورده است؛ CI/Readback محافظت‌شدهٔ تازه باقی است.
- CD-10 عملیات Corevia Bridge: قرارداد ثبت‌شده؛ Executor انتشار شاخه پیاده و Plan شده است، اما Executor اختصاصی نصب/سلامت/به‌روزرسانی/Rollback محصول هنوز `extension-needed` است.
- CD-11 پیگیری موبایل: طبق تصمیم، تعویق‌داده‌شده.
- CD-12 راستی‌آزمایی انتشار دسکتاپ WebView: برای تحویل نهایی باز است؛ Host بومی،
  Bundle مک، Smoke تصویری، اتصال واقعی ID/Relay، بررسی Full Desk ویندوز، گیت P1
  دو مقصد مستقل و artifact Customer Support Windows انجام شده، اما Install جانبی
  و UI تصویری Customer Support، GUI زندهٔ P3 و Readback محافظت‌شدهٔ Image/package
  در Nexus باقی است.

## گیت تشخیص Pattern

- `roadmap.create-or-update`: این رودمپ و جفت دوزبانه‌ی آن.
- `repo-specs.create-or-update`: مبنای OpenSpec و TestSpec.
- `mcp-service.create-or-migrate`: سورس MCP موجود پیدا نشد؛ کار آینده به `mcp-source.create-or-update` می‌رود.
- `corevia-bridge`: کار عملیاتی باید قبل از هر تغییر استقرار، کاتالوگ سورس، بررسی قابلیت و حاکمیت عملیات را پاس کند.

## مالکیت

| بخش | مالک | مرز مسئولیت |
| --- | --- | --- |
| قراردادها و Validatorهای مشترک | `corevia-standards` | قواعد عمومی، Schema، Evidence و گیت‌های ایمنی |
| Rust Core، ACL، پروفایل‌ها، Nuxt و MCP | `corevia-desk` | رفتار محصول و دسترسی به قابلیت‌های تأییدشده |
| قالب خام ظاهری | `corevia-desk-ui` | فقط منبع طراحی، نه رابط اجرایی |
| ساخت و انتشار بسته | CI/CD خود `corevia-desk` | ساخت، تست، امضای اختیاری و انتشار بسته‌های تغییرناپذیر |
| Nexus و عملیات سروری | `corevia-bridge` | فقط چرخه‌ی عملیاتی، بدون منطق محصول |
| Installer اختیاری مشتری | `corevia-bootstrap-installer` | فقط پروفایل Customer Support |

## فازها

### CD-1 — آماده‌سازی استانداردهای Corevia

وضعیت: کامل.

نتیجه:

- Corevia Desk به‌عنوان یک مخزن کنترل‌شده‌ی Corevia با منبع محلی و نسخه‌دار Standards ثبت شده است.

خروجی‌ها:

- `.corevia/repo.yaml`، `.corevia/follows.yaml`، `.corevia/skills.yaml` و `.corevia/validators.yaml`.
- OpenSpec، TestSpec و رودمپ دوزبانه.
- Values سورس‌محور برای کنترل مشخصات repo و رودمپ.

پذیرش:

- متادیتای repo و مستندات لازم وجود دارند.
- Validator استانداردها و بررسی Secret خام بدون خطا پاس می‌شوند.

صحت‌سنجی:

- `ruby ../corevia-standards/validators/corevia-standards/validate.rb --target . --check-repo`
- `git diff --check`

Evidence:

- متادیتای repo، خروجی Validator و `docs/evidence/corevia-desk-phase-1-2.fa.md`.

### CD-2 — مرزبندی و تصمیم معماری

وضعیت: کامل.

نتیجه:

- مالکیت و جهت وابستگی Rust، Nuxt/WebView، MCP، Nexus، Installer و Bridge صریح شده است.

خروجی‌ها:

- Decision Record معماری در `docs/decisions/`.
- ثبت Rust Core، ACL اجباری، View Layer وب، MCP، پروفایل‌های انتشار و مرز عملیات در OpenSpec و این رودمپ انجام شده و منابع رابط بومی قدیمی فقط حفظی هستند.

پذیرش:

- `corevia-desk-ui` منبع خام طراحی و `corevia-desk/web` لایه‌ی اجرایی Nuxt است.
- `corevia-web` خارج از مسیر رابط Corevia Desk است.
- اولویت با دسکتاپ است و موبایل تعویق دارد.
- Full Desk و Customer Support مرز انتشار و توزیع جدا دارند.

صحت‌سنجی:

- Decision Record، OpenSpec، TestSpec و رودمپ به‌عنوان یک نقشه‌ی وابستگی بررسی شوند.
- Validator استانداردهای repo و `git diff --check` اجرا شوند.

Evidence:

- `docs/decisions/corevia-desk-architecture.md`، جفت فارسی آن و `docs/evidence/corevia-desk-phase-1-2.fa.md`.

### CD-3 — ACL مشترک Rust و مرز دستورات

وضعیت: کامل.

نتیجه:

- Rust حالا یک مرز typed و مشترک برای مجوزها دارد که مسیر فعال Nuxt/WebView و Adapterهای MCP از آن استفاده می‌کنند؛ مسیرهای بومی حفظ‌شده برای ایمنی Rollback همچنان محافظت می‌شوند.

خروجی‌ها:

- `src/corevia_acl.rs` با پروفایل‌های Customer Support و Full Desk، قابلیت‌های نام‌دار، دلایل امن رد درخواست، مرجع تأیید و Audit بدون اطلاعات حساس.
- Feature به نام `corevia-customer-support`؛ Build پیش‌فرض همچنان Full Desk است.
- تنظیمات، تأیید و پرمیژن Connection Manager، Chat و عملیات Session بومی حفظ‌شده قبل از اجرا از مرز Rust عبور می‌کنند.
- قابلیت اجرای مستقیم دستور سیستم برای همه‌ی پروفایل‌ها رد می‌شود.

پذیرش:

- مسیر فعال Nuxt/WebView و مسیرهای بومی حفظ‌شده برای عملیات حساس پوشش‌داده‌شده بدون عبور از ACL Rust به عمل نمی‌رسند.
- Customer Support در این مرز فقط تنظیمات مشتری، تأیید اتصال ورودی و مجوزهای اتصال را دارد.
- عملیات حساس به مرجع معتبر نیاز دارند و یک Audit شماره‌دار و بدون اطلاعات حساس می‌سازند.
- سطح‌های Nuxt و MCP در همان مرز Rust نام‌گذاری شده‌اند؛ Executor بومی آن‌ها جداگانه و کنترل‌شده می‌ماند.

صحت‌سنجی:

- `git diff --check` — PASS.
- Validator مخزن Corevia Standards — PASS.
- بازبینی استاتیک همه‌ی مسیرهای بومی حفظ‌شده‌ی پوشش‌داده‌شده — PASS.
- تست‌های Rust در TestSpec تعریف شده‌اند، اما به‌دلیل نبود `cargo` و `rustc` روی این میزبان اجرا نشدند.

Evidence:

- `docs/evidence/corevia-desk-phase-3.md` و `docs/evidence/corevia-desk-phase-3.fa.md`.

### CD-4 — لایه‌ی View مبتنی بر Atomic Design در Nuxt

وضعیت: کامل.

نتیجه:

- قالب تأییدشده‌ی `corevia-desk-ui` به سیستم واقعی کامپوننت‌های Nuxt در `corevia-desk/web` تبدیل شود.

خروجی‌ها:

- برنامه‌ی Nuxt 3 در `web/` با دارایی‌های ظاهری قالب خام Corevia Desk در همین لایه‌ی اجرایی.
- کامپوننت‌های اتمیک در `components/atoms`، `components/molecules`، `components/organisms` و `components/templates`.
- سطوح مشترک Workspace، Dashboard، Devices، Sessions و Settings با حالت فارسی/انگلیسی و تاریک/روشن.
- مسیرهای صریح Full Desk و Customer Support. در Customer Support اتصال خروجی، دفترچه‌ی دستگاه‌ها و نشست‌ها نمایش داده نمی‌شوند.
- اسکریپت `web/scripts/validate-atomic-design.mjs` و `web/README.md` برای ثبت و بررسی مرز ترکیب لایه‌ها.

ساختار اجباری:

```text
web/
  components/
    atoms/
    molecules/
    organisms/
    templates/
  layouts/
  pages/
```

پذیرش:

- Pageها از Template، Templateها از Organism و لایه‌های پایین‌تر ساخته شوند.
- قواعد پایه‌ی رابط در سطح Page دوباره‌کاری نشوند.
- منبع طراحی همچنان `corevia-desk-ui` و رابط اجرایی فقط در `corevia-desk/web` باشد.
- رابط وب فعلاً داده‌ی نمونه داشته باشد، از ACL Rust عبور نکند و کامل‌بودن پل Nuxt به Rust را ادعا نکند.

صحت‌سنجی:

- `npm run lint:atomic` در `web/` — PASS.
- `npm run build` در `web/` با پروفایل پیش‌فرض Full Desk — PASS.
- `COREVIA_DESK_PROFILE=customer-support npm run build` — PASS.
- بررسی کوتاه در مرورگر محلی برای Full Desk، Customer Support، Settings، Devices، Sessions، تغییر زبان/جهت و پنجره‌ی مجوز — PASS.
- `git diff --check` — PASS.

### CD-5 — پل Nuxt به Rust

وضعیت: پیاده‌سازی‌شده — قرارداد، Adapter مرورگر و اتصال Host بومی آماده است.

نتیجه:

- Nuxt از طریق یک پل جدا به قابلیت‌های مجاز Rust دسترسی بگیرد و هیچ پل رابط قدیمی در مسیر فعال نباشد.

خروجی‌ها:

- `src/corevia_bridge.rs` یک کاتالوگ ثابت از عملیات را پیدا می‌کند و همه‌ی
  تصمیم‌ها را از `src/corevia_acl.rs` عبور می‌دهد.
- `docs/contracts/corevia-desk-bridge.schema.json` شکل Request، Response، نشست،
  رویداد و خطای امن را مشخص می‌کند.
- `web/composables/useCoreviaBridge.ts` از Bridge تزریق‌شده‌ی WebView/Agent محلی
  استفاده می‌کند و بدون دسترسی بومی، رابط خام را با Fixture نگه می‌دارد.

پذیرش:

- Nuxt فقط بعد از تصمیم ACL مشترک Rust پاسخ مجاز می‌گیرد و هیچ مسیر آزاد Rust،
  Shell، URL، SQL یا Repository در پل وجود ندارد.
- محدودیت مرورگر و نیاز به Agent محلی روشن است و Adapter به Bridge رابط قدیمی
  وابسته نیست.

صحت‌سنجی:

- Lint اتمیک Nuxt و Build تولیدی Full Desk — PASS.
- ماژول Rust و تست‌های مرز ACL ثبت شده‌اند؛ تست Host بومی و ACL در macOS arm64
  PASS شده‌اند.
- مرورگر همچنان همان طراحی خام را با داده‌ی نمونه نمایش می‌دهد.

### CD-5A — Host بومی WebView و اتصال Runtime

وضعیت: در macOS arm64 به‌صورت محلی پیاده‌سازی‌شده — بررسی Runner بومی و اجرای محافظت‌شده‌ی انتشار باقی است.

نتیجه:

- یک Host بومی مشخص در Rust، رابط Nuxt را داخل اپ دسکتاپ اجرا کند و آن را به
  پل موجود Rust وصل کند.

خروجی‌ها:

- Entry point بومی و وابستگی مشخص WebView در سورس ثبت و نگهداری شود.
- مسیر مطمئن برای بارگذاری `web/.output` در توسعه و بسته‌ی انتشار ایجاد شود.
- `window.__COREVIA_DESK_BRIDGE__` از طریق Host بومی تزریق شود و همه‌ی درخواست‌ها
  همچنان از ACL Rust عبور کنند.
- مسیرهای به‌جامانده‌ی Sciter/Flutter فقط برای نگهداری باقی بمانند و Launcher
  فعال دسکتاپ یا بسته‌ی انتشار آن‌ها را انتخاب نکند.

پذیرش:

- اجرای باینری دسکتاپ رابط Nuxt را باز کند، نه Sciter یا Flutter را.
- یک درخواست از صفحه‌ی Nuxt در حال اجرا به Rust برسد و نتیجه‌ی درست قبول یا رد
  را بدون Endpoint عمومی کنترل برگرداند.
- Full Desk و Customer Support هرکدام با سطح کامل یا محدود درست اجرا شوند.
- بسته‌ی انتشار همه‌ی اجزای لازم Host انتخاب‌شده را داشته باشد و فقط یک پوشه‌ی
  وب را کنار یک باینری نامرتبط قرار ندهد.

صحت‌سنجی:

- هر دو پروفایل روی اولین پلتفرم بومی Build و اجرا شوند؛ اولویت اثبات اولیه با
  macOS است. اکنون برای هر پروفایل یک `Corevia Desk.app` واقعی macOS arm64
  ساخته می‌شود.
- بازشدن پنجره، بارگذاری asset بسته، درخواست‌های Bridge، جدایی پروفایل‌ها،
  بستن/بازکردن و مجوزهای لازم دسکتاپ Smoke-test شدند. اجرای باینری، مشاهده‌ی
  تصویری، فریم واقعی و تست‌های Rust پل روی نشست باز macOS PASS شده‌اند.
- قبل از کامل اعلام‌شدن CD-12 همین بررسی‌ها روی Runner بومی Windows و Linux تکرار شوند.

### CD-6 — لایه‌ی کنترل MCP

وضعیت: پیاده‌سازی‌شده — Adapter محدود STDIO آماده؛ Binding بومی باقی مانده است.

نتیجه:

- MCP قابلیت‌های مجاز Rust را از طریق ابزارهای نام‌دار و مجوزدار کنترل کند.

خروجی‌ها:

- `mcp/src/catalog.mjs` کاتالوگ جدا برای Customer Support و Full Desk دارد.
- `mcp/src/policy.mjs` آرگومان‌ها و مرجع تأیید را قبل از صدا زدن Dispatch Rust
  تزریق‌شده بررسی می‌کند.
- `mcp/src/server.mjs` فقط `initialize`، `tools/list` و `tools/call` را دارد و
  Bridge پیش‌فرض `local-agent-required` برمی‌گرداند.

پذیرش:

- Proxy آزاد برای Rust، دستور سیستم، Endpoint، Repository یا SQL وجود نداشته باشد.
- Customer Support و Full Desk سیاست ابزار مؤثر جدا داشته باشند.

صحت‌سنجی:

- `npm --prefix mcp test` — PASS (۵ تست).
- جدایی پروفایل‌ها، الزام تأیید، رد ابزار ناشناخته و نبود Forward دستور سیستم
  بررسی شده است.

### CD-7 — CI/CD و خط انتشار Nexus

وضعیت: در محیط محلی پیاده‌سازی‌شده — Bundle قابل اجرای macOS arm64 برای هر دو پروفایل ساخته شد؛ imageهای پایهٔ CI برای `linux/amd64` از Source immutable به Nexus رسیدند و Readback شدند، اما CI محافظت‌شدهٔ جدید و انتشار Artifact محصول باقی است و امضا اختیاری است.

نتیجه:

- GitLab CI بسته‌های دسکتاپ نسخه‌دار و image سرور لینوکسی Full Desk را بسازد و
  در Nexus منتشر کند.

خروجی‌ها:

- `.gitlab-ci.yml` دقیقاً Matrix درخواستی را می‌سازد: Customer Support برای
  Windows و macOS؛ Full Desk برای Windows، macOS، Linux و image لینوکسی Docker.
- `release/build-profile.mjs` Rust و خروجی ایستای Nuxt را برای هر خانه‌ی Matrix
  می‌سازد و برای macOS یک `.app` واقعی تولید می‌کند؛ `archive-profile.mjs` بسته‌ی
  انتقال را تولید می‌کند.
- ابزارهای Manifest و بررسی، هویت تغییرناپذیر، checksum و گیت تأیید را می‌سازند؛
  امضا اختیاری است و CI آن را اجباری نمی‌کند.
- `release/publish-nexus.mjs` جایگزینی مسیر نسخه‌دار موجود را رد می‌کند.
- Readback رسمی Pipeline پس از تأمین image نشان داد `node:22-bookworm` با موفقیت Pull شده است؛ اجرای موجود روی commit قدیمی `2fc0e57f` در `verify:contracts` به‌دلیل ناسازگاری `package.json` و `package-lock.json` در `npm ci` شکست خورد. برای commit فعلی هنوز اجرای رسمی جدید و Readback کامل Artifact وجود ندارد؛ image داخلی نامرتبط جایگزین نشده است.

پذیرش:

- Customer Support بسته‌ی جدا برای Windows و macOS دارد. Full Desk بسته‌ی جدا
  برای Windows، macOS و Linux و همچنین image Docker لینوکس دارد.
- هر بسته سیستم‌عامل، معماری، نسخه، checksum و پروفایل انتشار مشخص داشته باشد.

صحت‌سنجی:

- `npm --prefix release test` — PASS.
- همه‌ی Scriptهای انتشار از نظر Syntax بررسی شدند و Secretهای Nexus و در صورت
  فعال‌شدن امضا، Secret امضا را فقط از CI می‌گیرند؛ هیچ Secretی در مخزن نیست.
- Build بومی macOS arm64 و بررسی Manifest برای هر دو پروفایل PASS شد. Windows release path نیز با `fileURLToPath`/`npm.cmd` اصلاح و تست شد. Resolver/Publish/Readback رسمی دو image پایهٔ CI برای `linux/amd64` PASS شد؛ اجرای محافظت‌شدهٔ جدید CI و انتشار Artifact محصول ادعا نمی‌شود و به‌عنوان گیت بیرونی باقی می‌ماند.

### CD-8 — محصول Customer Support

وضعیت: در محیط محلی پیاده‌سازی‌شده — پروفایل، UI، ACL، MCP و قرارداد انتشار بررسی شدند؛ انتشار Nexus محافظت‌شده بیرونی است.

نتیجه:

- بسته‌ی محدود مشتری فقط تنظیمات مربوط به مشتری، تأیید اتصال و مجوزهای اتصال را ارائه کند.

پذیرش:

- اتصال به سیستم نامرتبط، انتقال فایل آزاد، ترمینال یا دستور دلخواه سیستم ارائه نشود.

صحت‌سنجی:

- گیت سطح UI نسخه‌ی Customer Support — PASS.
- تست پروفایل MCP — PASS؛ اتصال خروجی و مجوزهای اضافی اتصال رد می‌شوند.
- قرارداد ACL جداگانه‌ی Customer Support در Rust ثبت شد؛ تست‌های هدف‌دار Native
  Cargo این پروفایل و Full Desk، به‌ترتیب ۱۱ تست Bridge و ۹ تست ACL، PASS شدند؛
  native Windows build، archive، manifest، smoke، process launch و MCP headless
  نیز PASS هستند، اما UI Automation خروجی قابل اتکا نداد و Install جانبی/UI
  تصویری هنوز باز است.

### CD-9 — محصول Full Desk

وضعیت: در محیط محلی پیاده‌سازی‌شده — پروفایل Full Desk، کاتالوگ کامل MCP و هویت انتشار مستقل بررسی شدند؛ انتشار Nexus محافظت‌شده بیرونی است.

نتیجه:

- بسته‌ی مستقل Full Desk تمام قابلیت‌های تأییدشده‌ی RustDesk و کاتالوگ کامل MCP را ارائه کند.

پذیرش:

- Full Desk آرتیفکت و مسیر انتشار مستقل در Nexus داشته باشد.
- عملیات پرخطر همچنان Approval و Audit داشته باشند.

صحت‌سنجی:

- قرارداد قابلیت‌ها و Approval نسخه‌ی Full Desk — PASS.
- قرارداد انتشار، Full Desk را از Customer Support جدا نگه می‌دارد.

### CD-10 — عملیات Corevia Bridge

وضعیت: قرارداد آماده — چرخه‌ی سورس‌محور ثبت شده است؛ Executor اختصاصی محصول هنوز `extension-needed` است.

نتیجه:

- نصب سروری، سلامت، به‌روزرسانی، Rollback و تنظیمات Runtime سورس‌محور و دارای Evidence باشند.

پذیرش:

- Bridge مالک Rust، Nuxt، MCP یا مجوزهای محصول نباشد.
- نبود Executor رسمی با `extension-needed` گزارش شود و Mutation دستی جایگزین نشود.

صحت‌سنجی:

- قرارداد چرخه‌ی Bridge هویت Nexus، سلامت، به‌روزرسانی، Rollback و Readback تنظیمات Runtime را بدون Credential ثبت می‌کند.
- بدون Executor متناظر Corevia Bridge هیچ Mutation بیرونی ادعا نمی‌شود.

### CD-11 — پیگیری موبایل

وضعیت: تعویق‌داده‌شده.

نتیجه:

- بسته‌بندی Android و iOS بعد از پایدارشدن مسیر دسکتاپ، مرورگر، ACL، MCP و Nexus دوباره بررسی شود.

پذیرش:

- موبایل باعث تأخیر دسکتاپ یا مرورگر نشود.

### Phase CD-12 — راستی‌آزمایی انتشار دسکتاپ WebView

Status: Open — تحویل نهایی هنوز کامل نیست.

وضعیت: Host بومی، Bundle مک، بررسی‌های Full Desk ویندوز و Artifact/ACL/MCP بومی
Customer Support Windows محلی تأیید شده‌اند. آزمون ID/Relay با تصویر واقعی
Windows، تأیید مقصد و کنترل‌های Remote Cursor/View Only موفق بوده؛ انتقال
Clipboard و درخواست‌های مستقل `/r` به Windows و Customer Support نیز به دو نشست
هم‌زمانِ تأییدشده با `direct:false` و `secured:true` رسیده‌اند. این آزمون‌های
اتصال موفق‌اند. در مک نیز پس از اجرای دوباره و آوردن Customer Support به جلو، سه
درخواست پیاپی بدون جابه‌جایی صفحه در ۱٫۶۴۴، ۱٫۴۲۵ و ۱٫۶۶۸ ثانیه ظاهر و رد شدند؛
یک تلاش حدود شش‌ثانیه‌ای پیش از اجرای دوباره به‌دلیل نامشخص‌بودن visibility مبهم
ماند. این تست‌ها P0 یا ماتریس کامل UI/دادهٔ P1 را نمی‌بندند. UI و نصب کنارهم
Customer Support ویندوز، تصویر/ورودی در نشست تأییدشده، بستن/اتصال مجدد، Secret
مثبت، GUI زندهٔ P3، Readback محافظت‌شدهٔ CI/Nexus و بررسی لینوکس باقی مانده‌اند.

Outcome:

- مسیر فعال دسکتاپ Rust به‌علاوه‌ی Nuxt داخل WebView بومی است و دو پروفایل انتشار جدا دارد.

Deliverables:

- بسته‌های پایدار و جداگانهٔ Full Desk و Customer Support برای مک و ویندوز، همراه با ماتریس پذیرش تأییدشده.
- شواهد رفتار UI/داده، MCP، اجرای تک‌نمونه و انتشار کنترل‌شدهٔ CI/Nexus.

Tasks:

- [x] نمایش خودکار درخواست ورودی Customer Support پس از اصلاح با ID/Relay زنده تأیید شد: پس از اجرای دوباره و آوردن Customer Support مک به جلو، سه درخواست پیاپی بدون جابه‌جایی مسیر در ۱٫۶۴۴، ۱٫۴۲۵ و ۱٫۶۶۸ ثانیه ظاهر شدند و هر سه رد شدند؛ هیچ دسترسی‌ای داده نشد. مورد مبهمِ حدود شش‌ثانیه‌ای پیش از اجرای دوباره نیز در Evidence حفظ شود.
- [ ] ماتریس بومی UI، داده، ماندگاری و حالت‌های خطا/خالی/درحال‌بارگذاری را برای هر دو پروفایل روی مک و ویندوز کامل کن؛ نصب و UI قابل مشاهدهٔ Customer Support را هم پوشش بده.
- [ ] روی ویندوز، اجرای دوباره، بازگردانی پنجره و اجرای کنارهمِ هر دو پروفایل را برای جلوگیری از نمونهٔ تکراری بررسی کن.
- [ ] ماتریس MCP سورس جاری شامل تأیید، رد، Audit، Elicitation و Executorها را برای هر دو پروفایل کامل کن. timeout واقعی ۱۲۰٫۰۴۲ ثانیه‌ایِ میزبان محلی stdio بدون اجرای Native Bridge امن بسته شد؛ تصمیم‌های تعاملی SDK، Audit، عملیات حساس بومی و بعضی Executorها بازند.
- [ ] وضعیت اعلان‌ها و خواندن مثبت Secret کنترل‌شده را در چهار گونهٔ دسکتاپ بررسی کن، بدون ثبت محتوای Secret.
- [ ] بلاکر Pipeline دقیقاً مربوط به commit جاری و Nexus را فقط از Corevia Standards → Corevia Bridge → `connectcore` حل کن؛ انتشار و Readback بسته‌های immutable فقط با Executor اعلام‌شده انجام شود.
- [ ] پس از پاس‌شدن چهار گونهٔ دسکتاپ، Build و بررسی Runtime، معماری و Image مربوط به Full Desk Linux و Docker را کامل کن.

Acceptance:

- هدف‌های Linux، macOS و Windows در CI وجود دارند؛ macOS arm64 در قرارداد انتشار صریحاً پشتیبانی می‌شود.
- هر Target باید اپ واقعی Nuxt داخل WebView را اجرا کند، نه فقط یک باینری Rust
  و یک پوشه‌ی وبِ جدا از آن.
- Manifest با تطابق پروفایل، سیستم‌عامل، معماری، نسخه، Runtime و checksum معتبر
  است؛ امضا فقط در Release امضاشده اختیاری است.
- Flutter و Dart در Jobهای فعال Build/Test نصب، اجرا یا لازم نیستند.

Verification:

- `npm --prefix release test` — PASS (۸ تست).
- `npm --prefix release run validate-scope` — PASS.
- Build تولیدی Nuxt برای Full Desk و Customer Support — PASS.
- Build بومی Rust، assetهای بسته‌ی Nuxt و تست بومی Bridge — PASS در macOS arm64.
- انتخاب Relay صریح با `/r` برای مقصد Windows و Customer Support تأیید و Active شد؛
  هر دو نشست مستقل با `direct:false` و `secured:true` هم‌زمان Readback شدند، بیش از
  ۲۰ ثانیه پایدار ماندند و سپس تمیز بسته شدند.
- Smoke-test تصویری GUI و کنترل واقعی نشست — PASS روی مک باز در برابر Windows
  Console.
- بعد از Build Release و نصب باینری جدید مک، درخواست دوباره‌ی همان شناسه به
  نشست موجود برگشت و نشست دوم نساخت؛ MCP فقط یک نشست فعال را گزارش کرد.
- تست هدف‌دار `cargo test --locked --release --lib corevia_bridge` — PASS (۱۱ تست) و `corevia_acl` — PASS (۹ تست)؛ همین دو گروه با Feature Customer Support نیز ۱۱/۹ PASS شدند.
- Build و Generate Nuxt، Smoke و Manifest macOS arm64 برای هر دو پروفایل — PASS.
- Manifest و native executable smoke مربوط به Customer Support Windows — PASS روی
  Runner ویندوز؛ اجرای PE روی macOS با `ENOEXEC` قابل قبول نیست و به‌عنوان UI
  Windows شمرده نمی‌شود.
- Native MCP هر دو پروفایل مک — PASS برای ۲۵/۹ ابزار، Readهای امن، Mutation مجاز،
  Approval، Audit، عدم اجرای نشست ناموجود و denyهای پروفایلی/حساس؛ P2 در مرز
  محصول و Native محلی بسته است.
- Build/Smoke-test واقعی Full Desk ویندوز روی مقصد انجام شد؛ انتشار محافظت‌شده
  CI/Nexus و Smoke-test لینوکس هنوز باز است. لینوکس مانع تحویل مک/ویندوز نیست.

Evidence:

- `docs/evidence/current-status.fa.md`
- `docs/evidence/corevia-desk-p1-ui-data.fa.md`
- `docs/evidence/corevia-desk-single-instance.fa.md`
- گزارش‌های بومی، MCP، CI و Nexus پالایش‌شده در مسیرهای شواهد اعلام‌شدهٔ `.work`.

## اعتبارسنجی

- Validator مخزن Corevia Standards.
- Validate و Plan مقادیر `roadmap.create-or-update`.
- Validate و Plan مقادیر `repo-specs.create-or-update`.
- `git diff --check`.
- قبل از فازهای بعدی، اجرای گیت‌های قرارداد MCP و کاتالوگ/قابلیت/حاکمیت Bridge.
- گیت Scope فعال Rust + Nuxt/WebView و سطح پروفایل‌ها.
- قبل از کامل اعلام‌شدن CD-12، تست اجرای بومی و Bridge انجام شود.

## سیاست به‌روزرسانی

مجاز:

- اصلاح Taskها، Acceptance و Evidence بر اساس یافته‌های تأییدشده.
- اضافه‌کردن جزئیات اجرایی بدون تغییر مالکیت یا معماری انتخاب‌شده.

نیازمند تصمیم جدید:

- تغییر مرز مالکیت Rust Core، ACL، Nuxt، MCP، Nexus، Installer یا Bridge.
- جلوآوردن موبایل از اولویت دسکتاپ.
- اضافه‌کردن وابستگی اجباری به Gateway مرکزی یا Endpoint عمومی کنترل.

## تاریخچه تغییرات

- ۲۰۲۶-۱۰-۰۵: پس از اصلاح همگام‌سازی تم/زبان Startup، بازبینی UI وضعیت ساختگی مجوزهای سیستم و دکمه‌های بی‌عمل را نیز پیدا کرد؛ راهنمای صادقانه در Source اصلاح شد و Profile/Atomic lint و Build وب PASS شدند. Readback Bundle بومی تازه برای هر دو اصلاح لازم است. فضای Data مک با انتقال قابل‌بازگشت cache آپدیت VS Code به Personal از ۱٫۸ به ۳٫۷ GiB رسید؛ فایل‌های سیستمی و cache فعال Chrome پاک نشدند.
- ۲۰۲۶-۱۰-۰۵ ادامه: دیالوگ اتصال Full Desk مک اصلاح و در Bundle `test11` تصویری/دسترس‌پذیری آزموده شد؛ Smoke و Release (۱۰/۱۰)، MCP (۲۲/۲۲) و Rust (۱۲۵/۱۲۵ برای هر پروفایل) PASS هستند. target موقت ۲٫۱ GiB همین Build پاک شد؛ Bundle ۲۴ MB حفظ شد و فضای آزاد روی `/` و Personal حدود ۵۳ GiB است. Docker image/cache دست‌نخورده ماندند. ماتریس پذیرش همچنان باز است.
- ۲۰۲۶-۱۰-۰۴: هدف تحویل چهار نسخهٔ مک/ویندوز قفل شد؛ SHA شاخه‌های Desk و Source Audit رسمی Bridge تأیید شدند. P1/MCP قدیمی به‌عنوان Evidence تاریخی علامت‌گذاری شد، بلاکر واقعی CI ثبت شد و Linux/Docker به اولویت پایین‌تر رفت.

- ۲۰۲۶-۰۹-۱۷: منبع رسمی Traefik برای Runner ویندوز reconcile و readback شد؛ HTTPS وضعیت ۳۰۲، API Runner وضعیت ۲۰۰ در مسیر پیش‌فرض/مستقیم، سرویس Running و `gitlab-runner verify` با `rc=0` ثبت شد. محیط vcpkg مک و پیش‌نیاز `libopus-dev` لینوکس نیز آماده و بررسی شدند؛ گیت انتشار commit فعلی و اجرای CI هنوز باز است.
- ۲۰۲۶-۰۹-۱۷ ادامه: pipeline رسمی `v1.5.21`، قراردادها و هر دو Build مک را پاس کرد؛ هر دو Build ویندوز در Checkout به‌دلیل `Filename too long` شکست خوردند. `core.longpaths=true` با Source/Executor رسمی روی Runner اعمال و Readback شد؛ commit/tag جدید و ماتریس کامل CI/Docker/Nexus هنوز باقی است.

- ۲۰۲۶-۰۹-۱۰: از مسیر Corevia Standards → Corevia Bridge → `connectcore`، Resolver Source برای Node `22-bookworm` و Debian `trixie-slim` digest index و digest پلتفرم `linux/amd64` را ثبت کرد؛ Publish رسمی هر دو به Nexus و Readback با digest مقصد دقیق پاس شد. شناسهٔ اجرای گزارش‌ها نیز برای اجرای موازی مستقل شد.
- ۲۰۲۶-۰۹-۱۰: Pipeline رسمی بعد از تأمین image Readback شد؛ Pull موفق بود و خطای باقی‌مانده در اجرای قدیمی `2fc0e57f`، ناسازگاری lockfile در `npm ci` است. Pattern/Executor رسمی انتشار شاخه ساخته و Plan شده؛ Apply تمیز، اجرای جدید روی commit فعلی و Artifact همچنان باز است.

- ۲۰۲۶-۰۹-۰۲: آماده‌سازی استانداردهای Corevia و مبنای معماری Rust Core، ACL، Nuxt، MCP، Nexus، Installer، Bridge و اولویت دسکتاپ ثبت شد.
- ۲۰۲۶-۰۹-۰۲: فاز CD-3 با ACL مشترک Rust، محدودیت Customer Support، پروفایل پیش‌فرض Full Desk، مرجع تأیید، Audit بدون اطلاعات حساس و اعمال روی مسیرهای Flutter/Sciter کامل شد.
- ۲۰۲۶-۰۹-۰۲: فاز CD-4 با ساخت لایه‌ی Nuxt مبتنی بر Atomic Design در `corevia-desk/web`، با منبع `corevia-desk-ui` و دو مسیر پروفایل محصول، همراه با Build موفق کامل شد.
- ۲۰۲۶-۰۹-۰۲: فازهای CD-5 تا CD-7 با قرارداد پل Rust، Adapter محدود MCP، Matrix انتشار دو پروفایل، گیت checksum و Publisher Nexus پیاده‌سازی شدند.
- ۲۰۲۶-۰۹-۰۳: تصمیم فعال دسکتاپ بر Rust به‌علاوه‌ی Nuxt/WebView تثبیت شد؛ منابع رابط قدیمی فقط حفظ شده‌اند و وارد Build، CI، Installer یا Release فعال نیستند.
- ۲۰۲۶-۰۹-۰۳: محدودیت Customer Support، قرارداد دو پروفایل، گیت Matrix پلتفرم و گیت‌های راستی‌آزمایی محلی CD-8 تا CD-12 پیاده‌سازی شدند.
- ۲۰۲۶-۰۹-۰۳: Host بومی Rust در CD-5A، بسته‌بندی ایستای Nuxt، Bundle مک و تست دو پروفایل پل ساخته و محلی بررسی شد؛ CD-12 تا تست GUI و Runnerهای بومی باز می‌ماند.
- ۲۰۲۶-۰۹-۰۵: باینری Release مک با گارد جلوگیری از نشست تکراری دوباره نصب شد؛ تصویر زنده، کنترل Start ویندوز، یک نشست فعال MCP و ۷ تست هدف‌دار Rust با موفقیت ثبت شدند.
- ۲۰۲۶-۰۹-۰۶: Source Audit رسمی پس از Catalog Write مجاز سبز شد، Sync شاخه‌ها با Fast-forward Readback شد و Evidence ماتریس P1 ثبت شد؛ Clipboard زنده و دو نشست هم‌زمان Active باز ماندند.
- ۲۰۲۶-۰۹-۰۷: تست تازهٔ Full Desk مک با Peer ID و ID/Relay به Windows به وضعیت Online و تصویر واقعی رسید؛ مقصد تأیید شد، UI تفاوت Relay را نشان داد، نشانگر راه‌دور/فقط‌مشاهده روشن و خاموش شدند و Clipboard واقعی دوطرفه Readback شد. فقط نشست هم‌زمان با دو مقصد مستقل باز ماند.
- ۲۰۲۶-۰۹-۰۷: منبع نشست ورودی/خروجی و دو Permission نمایش نشست در Bridge حفظ شدند؛ تست Bridge به ۱۱ و ACL به ۹ رسید و Archive/Manifest/Smoke هر دو Bundle مک دوباره PASS شدند.
- ۲۰۲۶-۰۹-۰۷: برای نصب هم‌زمان، شناسه و نام نمایشی Bundleهای Full Desk و Customer Support مک جدا شد؛ تست قرارداد Release به ۸ مورد رسید و `Info.plist` هر دو Bundle درست Readback شد. GUI زندهٔ Full Desk تم، جهت، مجوز و اعلان‌ها را نشان داد، اما GUI تصویری Customer Support و مثبت‌بودن Secret هنوز باز است.
- ۲۰۲۶-۰۹-۰۷: Bundle تازهٔ Customer Support مک با نام مستقل، حالت فقط‌پذیرش، Settings قفل‌شده و Clipboard/File Transfer/Audio غیرفعال Readback شد؛ انتقال واقعی Clipboard دوطرفه با marker زنده PASS شد و فقط Evidence نشست هم‌زمان P1 و UI تصویری Windows باز ماند.
- ۲۰۲۶-۰۹-۰۷: Native MCP هر دو پروفایل مک با کاتالوگ ۲۵/۹، Mutation، Approval، Audit و Deny readback شد؛ CD-6/P2 در محیط محلی کامل شد. namespace بومی و Peer ID دو پروفایل مک جدا شد؛ مقصد دوم تا Pending رسید، اما تأیید/Active هم‌زمان هنوز جای Evidence دو مقصد مستقل را نگرفته است.
- ۲۰۲۶-۰۹-۰۷: مسیر native `/r` با تأیید مقصد Windows به Active رسید و `direct:false` و `secured:true` Readback شد؛ Endpoint مستقل Customer Support در تلاش هم‌زمان آفلاین بود، پس گیت دو مقصد مستقل باز ماند. فضای دقیق build/cache مقصد آزاد شد، اما انتقال archive Customer Support از WinRM stream قابل اتکا نبود؛ پل‌ها، Processها، Task و فایل‌های انتقال موقت پاک شدند.
- ۲۰۲۶-۰۹-۰۷: گیت اصلی P1 با دو نشست مستقل `/r` برای Windows و Customer Support بسته شد؛ هر دو مقصد تأییدشده، Active، Relay و امن Readback شدند و پس از پایداری بیش از ۲۰ ثانیه تمیز بسته شدند. حالت headless کنترل‌شدهٔ سرور Customer Support برای میزبان مک قفل‌شده اضافه و فقط برای Runner استفاده شد؛ Install/UI ویندوز، Secret مثبت، Linux/Docker و CI/Nexus باز ماندند.

## Metadata

- وضعیت: Active
- مالک: Platform Engineering
- استاندارد: `corevia-standards@0.1.0`
- اولین کار باز فعلی: بازسازی و Readback بومی macOS برای اصلاحات Startup و راهنمای مجوز؛ سپس تکمیل UI/داده و نصب تصویری Customer Support روی Windows، بازآزمایی MCP روی Source فعلی، Secret مثبت و اجرای CI/Nexus جاری. Linux/Docker بعد از گیت‌های دسکتاپ می‌آید. Publish کنترل‌شدهٔ شاخه و Executor رسمی Bridge همچنان برای ترتیب `codex → develop → main` لازم است.
