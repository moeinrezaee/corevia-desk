[English](./corevia-desk-release-lifecycle.md)

# قرارداد چرخه‌ی انتشار Corevia Desk

## محدوده

این قرارداد مشخص می‌کند Corevia Bridge بعد از تولید Artifact توسط CI چطور می‌تواند
بسته‌های دسکتاپ Corevia Desk را مدیریت کند. مالک رفتار Rust، رابط Nuxt، مجوزهای
MCP یا تصمیم قابلیت‌های محصول نیست.

Runtime فعال دسکتاپ Rust به‌علاوه‌ی Nuxt داخل WebView بومی است. Flutter و Dart
خارج از مسیر فعال Build، Test، Installer و Release هستند.

## هویت تغییرناپذیر بسته

هر Release با این مسیر شناخته می‌شود:

```text
corevia-desk/<profile>/<version>/<platform>/<architecture>/
```

پروفایل‌های مجاز `customer-support` و `full-desk` هستند. سیستم‌عامل‌های مجاز
`linux`، `macos` و `windows` هستند؛ برای macOS معماری‌های `x86_64` و `arm64` و
برای اولین Matrix لینوکس و ویندوز معماری `x86_64` فعال است.

هر مسیر باید Archive و Manifest خودش را داشته باشد. فایل امضا برای Release
امضاشده می‌تواند اضافه شود، اما اجباری نیست. Manifest باید با Checksum بسته
یکی باشد و `rust`، `nuxt` و `rust-webview` را مشخص کند. مسیر نسخه‌دار Nexus
تغییرناپذیر است و نباید جایگزین شود.

## ترتیب کار Bridge

Executor اختصاصی محصول در آینده باید این ترتیب را از مسیر Corevia Standard و
`connectcore` اجرا کند:

1. پروفایل، نسخه، سیستم‌عامل و معماری را از Values سورس‌محور بخواند.
2. قبل از هر تغییر، Manifest دقیق و امضای اختیاری را از Nexus Readback کند.
3. Checksum را بررسی کند و اگر امضا وجود داشت آن را هم بررسی کند؛ بعد Artifact
   تغییرناپذیر را Acquire کند.
4. فقط Target اعلام‌شده را نصب یا به‌روزرسانی کند و نسخه‌ی سالم قبلی را به‌عنوان
   مرجع Rollback ثبت کند.
5. Health Checkهای اعلام‌شده را اجرا و نتیجه‌ی پاک‌سازی‌شده ثبت کند.
6. اگر Acceptance شکست خورد، بسته‌ی سالم قبلی را برگرداند و Health Check را تکرار کند.

SSH مستقیم، ویرایش دستی سرور، دانلود دلخواه یا Mutation از UI جایگزین Executor
ناقص نیست. نتیجه‌ی درست در این حالت `extension-needed` است.

## مرز فعلی

Corevia Bridge هم‌اکنون عملیات عمومی و کنترل‌شده‌ی Artifact و Nexus را دارد. اما
Executor اختصاصی نصب/Health/Rollback Corevia Desk هنوز در Catalog وجود ندارد؛ پس
CD-10 از نظر قرارداد آماده است ولی Mutation بیرونی آن تا ثبت و بررسی Executor
با وضعیت `extension-needed` باقی می‌ماند.
