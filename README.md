# 🇮🇷 persian-rs

[![Crates.io](https://img.shields.io/crates/v/persian-rs.svg)](https://crates.io/crates/persian-rs)
[![Docs.rs](https://docs.rs/persian-rs/badge.svg)](https://docs.rs/persian-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

یک جعبه ابزار جامع، سبک و کاربردی برای برنامه‌نویسان زبان Rust که با ابزارهای فارسی و ایرانی سروکار دارند. این کتابخانه هیچ وابستگی خارجی (dependency) ندارد و تمام الگوریتم‌ها به صورت بومی پیاده‌سازی شده‌اند.

---

## قابلیت‌ها

این کتابخانه در حال حاضر دو ماژول اصلی را ارائه می‌دهد:

### ماژول تبدیل تاریخ 
- **`to_jalali`**: تبدیل تاریخ میلادی به شمسی (جلالی).
- **`jalali_to_gregorian`**: تبدیل تاریخ شمسی (جلالی) به میلادی.

### ماژول اعتبارسنجی 
- **`is_valid_national_id`**: اعتبارسنجی صحت ساختاری کد ملی ایران.
- **`is_valid_card_number`**: اعتبارسنجی شماره کارت بانکی عضو شتاب بر اساس الگوریتم Luhn.

---


خط زیر را به فایل `Cargo.toml` خود اضافه کنید:
```toml
[dependencies]
persian-rs = "0.1.0" 
```


## مجوز

این پروژه تحت مجوز **MIT** منتشر شده است.


![Repo Badge](https://visitor-badge.laobi.icu/badge?page_id=null-err0r.persian-rs)
