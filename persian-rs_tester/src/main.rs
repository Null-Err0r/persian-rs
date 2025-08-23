use std::io; 

use persian_rs::{
    jalali::{jalali_to_gregorian, to_jalali},
    validation::{is_valid_card_number, is_valid_national_id},
};

fn main() {
    println!("🚀 برنامه تست کامل کتابخانه persian-rs 🚀");

    println!("\n------------------------------------");
    println!("## بخش تبدیل تاریخ میلادی به شمسی ##");
    
    println!("لطفاً سال میلادی را وارد کنید (مثلاً 2025):");
    let mut year_str = String::new();
    io::stdin().read_line(&mut year_str).expect("خطا در خواندن ورودی");
    let year: u16 = year_str.trim().parse().expect("لطفا عدد وارد کنید");

    println!("لطفاً ماه میلادی را وارد کنید (1-12):");
    let mut month_str = String::new();
    io::stdin().read_line(&mut month_str).expect("خطا در خواندن ورودی");
    let month: u16 = month_str.trim().parse().expect("لطفا عدد وارد کنید");

    println!("لطفاً روز میلادی را وارد کنید (1-31):");
    let mut day_str = String::new();
    io::stdin().read_line(&mut day_str).expect("خطا در خواندن ورودی");
    let day: u16 = day_str.trim().parse().expect("لطفا عدد وارد کنید");

    match to_jalali(day, month, year) {
        Ok(jalali_date) => {
            println!(
                "✅ تاریخ شمسی معادل: {}/{}/{}",
                jalali_date.year, jalali_date.month, jalali_date.day
            );
        }
        Err(e) => println!("❌ خطا: {}", e),
    }

    println!("\n------------------------------------");
    println!("## بخش تبدیل تاریخ شمسی به میلادی ##");

    println!("لطفاً سال شمسی را وارد کنید (مثلاً 1404):");
    let mut jyear_str = String::new();
    io::stdin().read_line(&mut jyear_str).expect("خطا در خواندن ورودی");
    let jyear: i32 = jyear_str.trim().parse().expect("لطفا عدد وارد کنید");

    println!("لطفاً ماه شمسی را وارد کنید (1-12):");
    let mut jmonth_str = String::new();
    io::stdin().read_line(&mut jmonth_str).expect("خطا در خواندن ورودی");
    let jmonth: i32 = jmonth_str.trim().parse().expect("لطفا عدد وارد کنید");

    println!("لطفاً روز شمسی را وارد کنید (1-31):");
    let mut jday_str = String::new();
    io::stdin().read_line(&mut jday_str).expect("خطا در خواندن ورودی");
    let jday: i32 = jday_str.trim().parse().expect("لطفا عدد وارد کنید");

    let gregorian_date = jalali_to_gregorian(jyear, jmonth, jday);
    println!(
        "✅ تاریخ میلادی معادل: {}/{}/{}",
        gregorian_date.year, gregorian_date.month, gregorian_date.day
    );

    println!("\n------------------------------------");
    println!("## بخش اعتبارسنجی ##");
    println!("\nلطفاً کد ملی خود را وارد کنید:");
    let mut national_id_input = String::new();
    io::stdin()
        .read_line(&mut national_id_input)
        .expect("خطا در خواندن ورودی");
    let trimmed_nid = national_id_input.trim();
    if is_valid_national_id(trimmed_nid) {
        println!("✅ کد ملی وارد شده معتبر است.");
    } else {
        println!("❌ کد ملی وارد شده نامعتبر است.");
    }

    println!("\nلطفاً شماره کارت بانکی خود را وارد کنید:");
    let mut card_number_input = String::new();
    io::stdin()
        .read_line(&mut card_number_input)
        .expect("خطا در خواندن ورودی");
    let trimmed_card = card_number_input.trim();
    if is_valid_card_number(trimmed_card) {
        println!("✅ شماره کارت وارد شده معتبر است.");
    } else {
        println!("❌ شماره کارت وارد شده نامعتبر است.");
    }

    println!("\n🎉 برنامه به پایان رسید. 🎉");
}
