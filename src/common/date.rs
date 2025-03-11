use chrono::{DateTime, NaiveDateTime, NaiveTime, TimeZone, Utc};

pub fn get_today_start_end_date() -> (DateTime<Utc>, DateTime<Utc>) {
    let now = Utc::now();
    let today = now.date_naive(); // 오늘 날짜 (NaiveDate)

    // 00:00:00 (시작 시간)
    let start_time = Utc.from_utc_datetime(&NaiveDateTime::new(
        today, 
        NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    ));

    // 23:59:59 (끝 시간)
    let end_time = Utc.from_utc_datetime(&NaiveDateTime::new(
        today, 
        NaiveTime::from_hms_opt(23, 59, 59).unwrap()
    ));

    (start_time, end_time)
}