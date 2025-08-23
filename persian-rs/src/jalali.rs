// src/jalali.rs


static DAY_SUM: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
static DAY_SUM_KABISE: [u16; 12] = [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

#[derive(PartialEq, Eq, Debug)]
pub struct JalaliDate {
    pub day: u16,
    pub year: u16,
    pub month: u16,
}

pub fn to_jalali(day: u16, month: u16, year: u16) -> Result<JalaliDate, &'static str> {
    if month > 12 || month < 1 {
        return Err("Month must be between 1 and 12");
    }
    if day > 31 || day < 1 {
        return Err("Day is not valid");
    }

    let is_gregorian_leap = is_gregorian_leap(year);
    let days_sum = if is_gregorian_leap {
        DAY_SUM_KABISE[month as usize - 1] + day
    } else {
        DAY_SUM[month as usize - 1] + day
    };

    let jalali_year = if days_sum < 80 { year - 622 } else { year - 621 };
    let is_jalali_leap = is_jalali_leap(jalali_year);

    let days_sum = if days_sum >= 80 {
        days_sum - 79
    } else {
        if is_gregorian_leap { days_sum + 287 } else { days_sum + 286 }
    };

    if days_sum <= 186 {
        let month = (days_sum as f32 / 31.0).ceil() as u16;
        let day = days_sum - ((month - 1) * 31);
        Ok(JalaliDate { day, year: jalali_year, month })
    } else {
        let remaining_days = days_sum - 186;
        let month = 6 + (remaining_days as f32 / 30.0).ceil() as u16;
        let day = remaining_days - ((month - 7) * 30);

        if month == 12 && day == 31 && !is_jalali_leap {
            return Ok(JalaliDate { day: 1, year: jalali_year + 1, month: 1 });
        }
        Ok(JalaliDate { day, year: jalali_year, month })
    }
}

fn is_gregorian_leap(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


fn is_jalali_leap(year: u16) -> bool {
    let rem = year % 33;
    matches!(rem, 1 | 5 | 9 | 13 | 17 | 22 | 26 | 30)
}


#[derive(Debug, PartialEq, Eq)]
pub struct GregorianDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

pub fn jalali_to_gregorian(year: i32, month: i32, day: i32) -> GregorianDate {
    let gy = year + 621;
    let is_initial_year_leap = is_gregorian_leap(gy as u16);
    
    let j_day_of_year = if month <= 6 {
        (month - 1) * 31 + day
    } else {
        186 + (month - 7) * 30 + day
    };

    let g_day_of_year = j_day_of_year + if is_initial_year_leap { 79 + 1 } else { 79 };
    
    let final_gy = if g_day_of_year > if is_gregorian_leap(gy as u16) { 366 } else { 365 } {
        gy + 1
    } else {
        gy
    };

    let remaining_days = g_day_of_year % if is_gregorian_leap(final_gy as u16) { 366 } else { 365 };
    let final_remaining_days = if remaining_days == 0 { if is_gregorian_leap(final_gy as u16) { 366 } else { 365 } } else { remaining_days };

    let mut month_of_year = 0;
    let days_in_month = if is_gregorian_leap(final_gy as u16) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    
    let mut temp_days = final_remaining_days;
    for (i, days) in days_in_month.iter().enumerate() {
        if temp_days <= *days {
            month_of_year = (i + 1) as i32;
            break;
        }
        temp_days -= days;
    }

    GregorianDate {
        year: final_gy,
        month: month_of_year,
        day: temp_days,
    }
}

