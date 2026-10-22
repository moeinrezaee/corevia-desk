# لایه‌ی وب Corevia Desk

این پوشه لایه‌ی رابط Nuxt برای Corevia Desk است. منبع ظاهری آن قالب خام
`../corevia-desk-ui` است؛ قالب خام جدا می‌ماند و برنامه‌ی اجرایی نیست.

## قاعده‌ی ترکیب

هر Page از یک Template ساخته می‌شود. Templateها Organism، Organismها Molecule
و Moleculeها Atom را می‌سازند:

```text
Atoms → Molecules → Organisms → Templates → Layouts → Pages
```

در Pageها کنترل‌های پایه دوباره نوشته نمی‌شوند و دسترسی مستقیم به لایه‌های
پایین‌تر وجود ندارد. دستور `lint:atomic` این مرز را بدون نصب بسته‌ها بررسی
می‌کند.

## دو نسخه

- `full-desk` نسخه‌ی پیش‌فرض است و فضای کامل دسکتاپ را نشان می‌دهد.
- `customer-support` فقط تنظیمات مربوط به مشتری، بررسی اتصال ورودی و انتخاب
  مجوزهای همان اتصال را نشان می‌دهد؛ اتصال خروجی، دفترچه‌ی دستگاه‌ها و نشست‌ها
  در آن وجود ندارد.

برای ساخت نسخه‌ی محدود از `COREVIA_DESK_PROFILE=customer-support` استفاده شود.
رابط در مرورگر داده‌ی نمونه را به‌عنوان مسیر امن نگه می‌دارد و درخواست‌های بومی
را از `useCoreviaBridge` می‌فرستد. میزبان دسکتاپ می‌تواند
`window.__COREVIA_DESK_BRIDGE__` را تزریق کند؛ این Binding باید قرارداد ACL
موجود در `../src/corevia_bridge.rs` را صدا بزند.

لایه‌ی وب داخل `corevia-desk` قرار دارد و رابط فعال دسکتاپ است؛ پوسته‌ی WebView
بومی Rust آن را اجرا می‌کند و Rust منبع اصلی دسترسی سیستم باقی می‌ماند. مسیر
فعال Build به نصب یا اجرای Flutter/Dart نیاز ندارد. قرارداد کامل در
`../docs/contracts/corevia-desk-bridge.fa.md` است و Bridge Nuxt به منابع رابط
قدیمی وابسته نیست.
