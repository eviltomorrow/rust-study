use crate::RoughTime;

pub(crate) fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{} {} ago", count, units.pluar())
        }
        RoughTime::JustNow => String::from("just now"),
        RoughTime::InTheFuture(units, 1) => {
            format!("{} {} from now", 1, units.sigular())
        }
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.pluar())
        }
    }
}
