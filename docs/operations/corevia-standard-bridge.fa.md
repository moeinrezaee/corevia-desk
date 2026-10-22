[English](./corevia-standard-bridge.md)

# مسیر عملیاتی Corevia Standard و Corevia Bridge

آخرین به‌روزرسانی: ۲۰۲۶-۱۰-۰۸
وضعیت: فعال
مالک: Platform Engineering

## هدف

ثبت می‌کند Corevia Desk چطور از قواعد Standards و مرز اجرای Bridge استفاده می‌کند بدون این‌که کار محصول به عملیات دستی و بدون سند تبدیل شود.

## محدوده

این سند حاکمیت Repo، اعتبارسنجی مستندات/اسپک، عملیات Branch گیت‌لب، عملیات Artifact/Image در Nexus و مرز کد محصول با زیرساخت عملیاتی را پوشش می‌دهد.

## پیش‌نیاز

- سورس‌های محلی `corevia-standards` و `corevia-bridge` کنار این مخزن در دسترس باشند.
- عملیات در یک Values سورس‌محور ثبت شده باشد.
- Executor و Referenceهای Secret لازم در دسترس باشند؛ Credential خام هرگز وارد Values نمی‌شود.

## پیکربندی

Intent مربوط به Repo در `.corevia/operations/` است. رفتار محصول در Corevia Desk و State مربوط به GitLab، Nexus و چرخه‌ی Host پشت Values و Executor اعلام‌شده‌ی Bridge می‌ماند.

## Readback آخرین سورس‌ها

پس از Fetch در ۲۰۲۶-۱۰-۰۸، refهای دوردست Desk یعنی
`codex/desk-phases-8-12`، `develop` و `main` همگی روی
`248afe1b69a73efe171c12d2714cd42a942fb04f` هستند؛ مقایسهٔ Fast-forward هر دو
مسیر ۰/۰ است، پس تغییری روی شاخه‌های دوردست لازم نبود. Working tree Desk و
`libs/hbb_common` کثیف‌اند؛ تغییرات محلی هنوز در آن سه ref نیستند و Commit/Push
نشدند.

Snapshotهای تمیز تازه: Standards `origin/develop` برابر `f663ba2`، Bridge
`origin/develop` برابر `7bb1645` و Knowledge `origin/develop` برابر `e605a19`؛
snapshot شاخهٔ `origin/main` در Knowledge برابر `539b3e9` باقی است. Source Audit
Bridge روی `7bb1645` ابتدا فقط یک Drift هش برای
`kernel/usecases/service_repo.sh` پیدا کرد (entry کاتالوگ کم یا اضافه نشده بود).
پس از بازبینی تغییر منبع، Validate/Plan رسمی Standards برای
`bridge-operation.govern` پاس شد؛ دستور رسمی `connectcore source catalog --write`
فقط هش همان جزء را در snapshot جداگانهٔ Bridge عوض کرد. سپس Source Audit و
Validate/Plan رسمی `bridge-source.catalog` با همان شمار ۱۸۰ playbook، ۱۹۶ role،
۱۶۸ use case، ۱۸۴ command، ۱۸۲ adapter خروجی، ۸ ماژول Terraform و ۳۲ مورد legacy
پاس شدند. این تغییر تک‌فایلی محلی و منتشرنشده است؛ شاخهٔ دوردست Bridge تغییر
نکرد.

Checkout اصلی Bridge، Standards و Knowledge Switch/Reset/Stash نشد. برای عملیات
بازبینی‌شده از snapshotهای `.work/*-latest` استفاده کن و همهٔ فایل‌های کثیف Desk
و submodule را حفظ کن.

## اجرا و استفاده

با گیت تشخیص Pattern شروع کن، `validate` و `plan` را اجرا کن، سپس فقط بعد از تأیید از Executor مشخص Bridge استفاده کن. در پایان Readback و Evidence پاک‌سازی‌شده بگیر.

## گیت تشخیص Pattern

قبل از هر عملیات تکرارشونده، آن را `reuse`، `extend`، `migrate-on-touch`، `create-new` یا `not-repeatable` مشخص کن.

| کار | Pattern | Values | مالک اجرا |
| --- | --- | --- | --- |
| رودمپ | `roadmap.create-or-update` | `.corevia/operations/roadmap.values.yaml` | Executor محلی Standards |
| Evidence فاز | `roadmap-phase.execute` | `.corevia/operations/roadmap-phase.values.yaml` | حاکمیت Standards/Bridge |
| اسپک Repo | `repo-specs.create-or-update` | `.corevia/operations/repo-specs.values.yaml` | Executor محلی Standards |
| Audit مستندات | `documentation.audit` | `.corevia/operations/documentation-audit.values.yaml` | Executor محلی Standards |
| Branch گیت‌لب | `gitlab-branch.reconcile` | `.corevia/operations/gitlab-branch-reconcile.values.yaml` | Corevia Bridge |
| همگام‌سازی Branch | `gitlab-branch.sync` | `.corevia/operations/gitlab-branch-sync.values.yaml` | Corevia Bridge |
| Branch محافظت‌شده | `gitlab-branch-protection.reconcile` | `.corevia/operations/gitlab-branch-protection.values.yaml` | Corevia Bridge |
| انتشار Image در Nexus | `container-image.publish` | sibling `corevia-bridge/values/platform/corevia-desk-gitlab-runner-image.yaml` | Corevia Bridge |
| Readback Image در Nexus | `container-image.nexus-readback` | sibling `corevia-bridge/values/platform/corevia-desk-gitlab-runner-image-readback.yaml` | Corevia Bridge |

### هشدار اسپک Repo

Pattern فعلی `repo-specs.create-or-update` در وضعیت draft است و فایل‌های OpenSpec و
TestSpec را از Values بازتولید می‌کند؛ ممکن است معیارهای پذیرش مفصلِ موجود را
حذف کند. Validate و Plan فقط‌خواندنی‌اند، اما اجرای Apply روی فایل‌های اصلی این
مخزن یک بار متن معیارها را ناقص کرد. فایل‌ها از سند مرجع پذیرش بازسازی و با
Validator بررسی شدند. تا وقتی Pattern طوری اصلاح نشده که محتوای ناشناخته را حفظ
کند و diff کامل بازبینی نشود، Apply دوباره اجرا نشود.

Plan رسمی هنوز Update هر دو فایل را پیشنهاد می‌کند؛ این پیش‌نمایش است و Apply
دیگری اجرا نشده است.

## فرمان‌های استاندارد

Documentation Audit را با snapshot جداگانه‌ی آخرین Standards اجرا کن. Values
مخزن Desk را نسبت به workspace در `.work` حل می‌کند؛ از مسیر
`/Volumes/Personal/Projects/corevia/.work` اجرا کن و فقط اگر وجود ندارند، لینک‌های
موقت مخزن را بساز؛ پس از اعتبارسنجی فقط همان لینک‌ها را حذف کن:

```sh
ln -s ../corevia-desk corevia-desk
ln -s corevia-bridge-latest corevia-bridge
./corevia-standards-latest/bin/corevia-run validate --pattern documentation.audit --values corevia-desk/.corevia/operations/documentation-audit.values.yaml
./corevia-standards-latest/bin/corevia-run plan --pattern documentation.audit --values corevia-desk/.corevia/operations/documentation-audit.values.yaml
unlink corevia-desk
unlink corevia-bridge
```

خروجی Plan و Evidence در مسیر `.work` اعلام‌شده نگه داشته می‌شود و Credential خام نباید در Values یا Report باشد.

## زنجیره‌ی اجرای Bridge

```text
Corevia Standards -> کاتالوگ سورس -> حل قابلیت -> Pattern مشخص
-> bridge-operation.govern -> connectcore -> Terraform/Ansible/provider -> Readback/Evidence
```

برای وضعیت GitLab یا Nexus، Values سورس‌محور درخواست هستند و Corevia Bridge Executor است. SSH مستقیم، تغییر دستی UI گیت‌لب، Mutation با WinRM یا Upload یک‌باره جایگزین معتبر Executor نیست. ترتیب همگام‌سازی Corevia Desk در Values ثبت شده است: `codex/desk-phases-8-12 -> develop` و سپس `develop -> main`، هر دو فقط `fast-forward-only`.

## شکاف فعلی Bridge

قابلیت‌های عمومی Branch گیت‌لب و Image Container اعلام شده‌اند. Executor اختصاصی نصب/سلامت/به‌روزرسانی/Rollback برای Corevia Desk هنوز `extension-needed` است و قرارداد انتشار ترتیب لازم آینده را ثبت کرده. این مورد P0 اتصال واقعی محلی را متوقف نمی‌کند، اما تحویل عملیاتی را باز نگه می‌دارد.

Audit روی snapshot جداگانهٔ Bridge پس از بازتولید رسمی کاتالوگ سبز است، اما این
تک‌فایل هنوز روی شاخهٔ دوردست Bridge منتشر نشده. در snapshot آخر Standards، Pattern
پایپ‌لاین محصول هنوز به فایل غایب
`corevia-bridge/values/platform/corevia-desk-gitlab-pipeline-current.yaml`
ارجاع می‌دهد و کاتالوگ تازه‌ی `connectcore` عملیات `gitlab-pipeline-current` را
اعلام نمی‌کند. بنابراین CI/Publish دستی اجرا نشد. Readback تصویر Runner در Nexus
سبز است، اما انتشار Artifact محصول و Executor اختصاصی آن کامل نیست. Readback با
WinRM یا SSH جایگزین Executor حاکمیت‌شده نیست.

Values موردارجاع درخواست `gitlab-branch.sync` در Desk نیز در Bridge غایب است.
این کار به Write دوردست نیاز نداشت، چون refهای fetched منبع Desk، `develop` و
`main` از قبل روی یک Commit هستند؛ Mutation همگام‌سازی اجرا نشد.

بازبینی رسمی تازه، Source Catalog و هر دو گیت Resolve/Extend و Operation Governance
را در Validate/Plan پاس کرد؛ تصمیم Extend است. فرمان موجود `platform:gitlab-pipeline`
فقط آخرین Pipeline ناموفق را retry می‌کند و برای اجرای commit دقیق جاری و Readback
تمام Job/Artifactها کافی نیست. Pattern محصول هنوز `draft` است؛ تا اضافه‌شدن Executor
و Values و رفع گیت تأیید پیش از اجرا، هیچ Pipeline یا Publish مجاز/اجراشده نیست.

## مدیریت Secret

در Values فقط Referenceهایی مثل `env://NEXUS_PASSWORD` مجاز است. Credential واقعی از Secret Store یا Secret Variableهای CI تأمین می‌شود و هرگز وارد چت، سند، Log، Manifest یا Commit نمی‌شود.

## اعتبارسنجی

- Validator مخزن Corevia Standards.
- Audit مستندات و بررسی Pair/Link محلی.
- `git diff --check`.
- Validate و Plan مربوط به هر Pattern قبل از Apply Bridge.
- Readback بعد از Apply و Evidence پاک‌سازی‌شده.

## رفع اشکال

- اگر Pattern یا Executor مناسب وجود نداشت، در `extension-needed` متوقف شو.
- اگر Branch محافظت‌شده Push مستقیم را رد کرد، مسیر Review/Merge اعلام‌شده را برو؛ Force Push ممنوع است.
- اگر Readback Nexus با Digest سورس اختلاف داشت، Fail Closed کن و مسیر نسخه‌دار را Overwrite نکن.

## تاریخچه تغییرات

- ۲۰۲۶-۱۰-۰۴: نسخه‌های Fetch‌شده و روش Snapshot ایمن برای checkoutهای کثیف Standards/Knowledge ثبت شد؛ Source Audit سبز Bridge تأیید و بلاکر دقیق Values/عملیات پایپ‌لاین جاری ثبت شد.
- ۲۰۲۶-۰۹-۰۶: Catalog Drift از مسیر رسمی رفع و Source Audit سبز شد؛ باقی‌ماندن شکاف Executor محصولی، انتشار Artifact و Readback CI/Nexus ثبت شد.
- ۲۰۲۶-۰۹-۰۵: نقشه‌ی عملیات Standard/Bridge، Values رسمی همگام‌سازی Branch و شکاف Executor اختصاصی Corevia Desk ثبت شد.
- ۲۰۲۶-۱۰-۰۸: خطر بازنویسی ناقص در Apply مربوط به `repo-specs.create-or-update` ثبت شد؛ OpenSpec و TestSpec از مرجع پذیرش بازسازی شدند. Plan هنوز Update هر دو فایل را پیشنهاد می‌کند؛ Apply دوباره ممنوع است تا الگو اصلاح و diff بازبینی شود.
- ۲۰۲۶-۱۰-۰۸: خطر بازنویسی ناقص در Apply مربوط به `repo-specs.create-or-update` ثبت شد؛ OpenSpec و TestSpec از مرجع پذیرش بازسازی و اعتبارسنجی شدند. Apply دوباره ممنوع است تا الگو اصلاح و diff کامل بازبینی شود.

## مالکیت

Corevia Standards مالک حاکمیت، Corevia Bridge مالک Mutation و Readback عملیاتی و Corevia Desk مالک سورس محصول و Intent انتشار است.
