use chrono::NaiveDate;

// начало выброса
pub static ACCIDENT_BEGIN: NaiveDate = NaiveDate::from_ymd_opt(1986, 4, 26).unwrap();
// конец выброса
pub static ACCIDENT_END: NaiveDate = NaiveDate::from_ymd_opt(1986, 5, 7).unwrap();

// время изменения диапазона высоты с высокой на низкую (часов после начала выброса)
pub static H_RANGE_CHANGING_TIME: u16 = 783;
pub static UPPER_H_RANGE: (f32, f32) = (600.0, 1000.0);
pub static LOWER_H_RANGE: (f32, f32) = (40.0, 600.0);
pub static SIZE_RANGE: (f32, f32) = (48.6, 52.4);
