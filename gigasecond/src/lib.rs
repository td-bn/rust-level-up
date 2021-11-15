use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start.checked_add(Duration::seconds(10_i64.pow(9))).unwrap()
}
