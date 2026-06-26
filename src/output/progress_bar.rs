use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::options::OutputStyleOption;

#[cfg(not(windows))]
const TICK_SETTINGS: (&str, u64) = ("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ", 80);

#[cfg(windows)]
const TICK_SETTINGS: (&str, u64) = (r"+-x| ", 200);

const DEFAULT_PROGRESS_TEMPLATE: &str = " {spinner} {msg:<30} {wide_bar} ETA {eta_precise} ";
const INITIAL_MEASUREMENT_TEMPLATE: &str = " {spinner} {msg} {elapsed_precise:<10} ";

fn progress_style(option: OutputStyleOption, template: &str) -> ProgressStyle {
    match option {
        OutputStyleOption::Basic | OutputStyleOption::Color => ProgressStyle::default_bar(),
        _ => ProgressStyle::default_spinner()
            .tick_chars(TICK_SETTINGS.0)
            .template(template)
            .expect("no template error"),
    }
}

/// Return a pre-configured progress bar
pub fn get_progress_bar(length: u64, msg: &str, option: OutputStyleOption) -> ProgressBar {
    make_progress_bar(length, msg, option, DEFAULT_PROGRESS_TEMPLATE)
}

pub(crate) fn get_initial_measurement_progress_bar(
    length: u64,
    msg: &str,
    option: OutputStyleOption,
) -> ProgressBar {
    make_progress_bar(length, msg, option, INITIAL_MEASUREMENT_TEMPLATE)
}

fn make_progress_bar(
    length: u64,
    msg: &str,
    option: OutputStyleOption,
    template: &str,
) -> ProgressBar {
    let progress_bar = match option {
        OutputStyleOption::Basic | OutputStyleOption::Color => ProgressBar::hidden(),
        _ => ProgressBar::new(length),
    };
    progress_bar.set_style(progress_style(option, template));
    progress_bar.set_message(msg.to_owned());
    progress_bar.enable_steady_tick(Duration::from_millis(TICK_SETTINGS.1));

    progress_bar
}

pub(crate) fn reset_progress_bar(bar: &ProgressBar, option: OutputStyleOption, msg: String) {
    bar.disable_steady_tick();
    bar.set_style(progress_style(option, DEFAULT_PROGRESS_TEMPLATE));
    bar.set_message(msg);
    bar.enable_steady_tick(Duration::from_millis(TICK_SETTINGS.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_measurement_template_omits_eta() {
        assert!(INITIAL_MEASUREMENT_TEMPLATE.contains("{elapsed_precise"));
        assert!(!INITIAL_MEASUREMENT_TEMPLATE.contains("{eta_precise}"));
        assert!(!INITIAL_MEASUREMENT_TEMPLATE.contains("{wide_bar}"));
    }

    #[test]
    fn default_progress_template_is_valid_after_reset() {
        let bar = get_initial_measurement_progress_bar(
            1,
            "Initial time measurement:",
            OutputStyleOption::Full,
        );
        reset_progress_bar(
            &bar,
            OutputStyleOption::Full,
            "Current estimate: 1.000 s".into(),
        );
    }
}
