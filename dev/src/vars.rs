use std::env::var;

pub fn gather() -> Option<(String, String, String)> {
    if let (Ok(start), Ok(end), Ok(wd)) = (
        var("START"), //
        var("END"),   //
        var("WD"),
    ) {
        return Some((start, end, wd));
    } else {
        return None;
    }
}
