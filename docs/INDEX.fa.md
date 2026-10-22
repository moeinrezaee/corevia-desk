[English](./INDEX.md)

# فهرست مستندات Corevia Desk

این فایل نقشه‌ی مرجع سورس، وضعیت واقعی و نقطه‌ی ادامه‌ی دقیق کار است. قبل از هر تغییر، وضعیت فعلی و handoff را بخوان.

## از اینجا شروع کن

۱. [وضعیت فعلی](./evidence/current-status.fa.md) — چه چیزی واقعاً تأیید شده است.
۲. [Handoff کانتکست بعدی](./handoff/next-context.fa.md) — متن آماده برای کانتکست خالی بعدی.
۳. [رودمپ](./roadmaps/roadmap.fa.md) — ترتیب فازها، مالکیت، پذیرش و کارهای باز.
۴. [گیت‌های پذیرش](./specs/acceptance.fa.md) — تعریف موفقیت هر اولویت.
۵. [تاریخچه‌ی مشکل‌ها](./incidents/corevia-desk-problem-history.fa.md) — خطاهای حل‌شده و ریسک‌های باز.

## معماری و قراردادها

- [تصمیم معماری](./decisions/corevia-desk-architecture.fa.md)
- [نمای کلی محصول](./specs/overview.fa.md)
- [قراردادهای رفتار و مالکیت](./specs/contracts.fa.md)
- [قرارداد پل Nuxt/Rust](./contracts/corevia-desk-bridge.fa.md)
- [Schema ماشینی پل](./contracts/corevia-desk-bridge.schema.json)
- [قرارداد چرخه‌ی انتشار](./contracts/corevia-desk-release-lifecycle.fa.md)

## اسپک و راستی‌آزمایی

- [OpenSpec](./specs/openspec.yaml)
- [TestSpec](./specs/testspec.yaml)
- [نمای کلی اسپک](./specs/overview.fa.md)
- [ماتریس پذیرش](./specs/acceptance.fa.md)
- [تغییرات اسپک](./specs/changelog.fa.md)
- [مدرک فازها](./evidence/corevia-desk-phase-1-2.fa.md)، [CD-3](./evidence/corevia-desk-phase-3.fa.md)، [CD-4](./evidence/corevia-desk-phase-4.fa.md)، [CD-5](./evidence/corevia-desk-phase-5.fa.md)، [CD-6](./evidence/corevia-desk-phase-6.fa.md)، [CD-7](./evidence/corevia-desk-phase-7.fa.md)، [CD-8 تا CD-12](./evidence/corevia-desk-phase-8-12.fa.md)، [ماتریس P1 UI و داده](./evidence/corevia-desk-p1-ui-data.fa.md)

## عملیات و انتشار

- [مسیر Corevia Standard و Bridge](./operations/corevia-standard-bridge.fa.md)
- [ابزارهای انتشار](../release/README.fa.md)
- [لایه‌ی Nuxt/WebView](../web/README.fa.md)
- [لایه‌ی MCP](../mcp/README.fa.md)
- [استاندارد مستندات](./standards/DOCUMENTATION_STANDARD.fa.md)
- Values خود repo: `.corevia/operations/`
- Values مربوط به Bridge: فایل‌های `corevia-desk-*` در sibling به نام `corevia-bridge/values/`

## مرزهای تغییرناپذیر

- محصول فعال فقط Rust Core به‌علاوه‌ی Nuxt داخل WebView بومی است.
- Full Desk و Customer Support پروفایل و هویت انتشار جدا دارند.
- اتصال محصول با شناسه‌ی RustDesk از مسیر ID/Relay سنجیده می‌شود؛ تست IP یا هم‌شبکه‌بودن جای آن نیست.
- MCP از همان ACL Rust استفاده می‌کند و جای محصول یا Proxy آزاد دستور سیستم نیست.
- Flutter، Dart و Sciter خارج از Scope فعال هستند.
- موبایل تعویق دارد و نباید هدف دسکتاپ را متوقف کند.

## مالکیت

Corevia Desk مالک رفتار محصول، ACL Rust، Nuxt، Adapter MCP و تعریف انتشار است. Corevia Standards مالک حاکمیت مشترک است. Corevia Bridge فقط مالک نصب عملیاتی تأییدشده، سلامت، به‌روزرسانی، Rollback و چرخه‌ی Nexus است.
