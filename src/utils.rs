use chrono::Local;

pub fn get_current_date() -> String {
    let date = Local::now();
    date.format("%B %d, %Y").to_string()
}
