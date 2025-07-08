use chrono::NaiveDateTime;

/// Парсит объект даты-времени, полученный из запроса
pub fn get_datetime(input_datetime: &str) -> Result<NaiveDateTime, String> {
    let err_return = format!("Unable to parse input string: {input_datetime}");
    let iso_datetime = match iso8601::datetime(&input_datetime) {
        Ok(datetime) => datetime,
        Err(_) => return Err(err_return),
    };
    let naive_datetime = match iso_datetime.into_naive() {
        Some(datetime) => datetime,
        None => return Err(err_return),
    };
    Ok(naive_datetime)
}
