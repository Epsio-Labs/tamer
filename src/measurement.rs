use std::time::Duration;

pub(crate) struct DurationFormatter;
impl DurationFormatter {
    pub(crate) fn elements_per_second(elems: f64, duration: Duration) -> String {
        let elems_per_second = elems / duration.as_secs_f64();
        let (denominator, unit) = if elems_per_second < 1000.0 {
            (1.0, " elem/s")
        } else if elems_per_second < 1000.0 * 1000.0 {
            (1000.0, "Kelem/s")
        } else if elems_per_second < 1000.0 * 1000.0 * 1000.0 {
            (1000.0 * 1000.0, "Melem/s")
        } else {
            (1000.0 * 1000.0 * 1000.0, "Gelem/s")
        };

        format!("{:.2}{}", elems_per_second / denominator, unit)
    }
}