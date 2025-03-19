use chrono::Datelike;
use chrono::Timelike;
use chrono::{DateTime, Local, NaiveTime};

#[derive(Debug)]
struct DateParserError {
    message: String,
}

impl std::fmt::Display for DateParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DateParserError: {}", self.message)
    }
}

fn parse_date(input: &str) -> Result<DateTime<Local>, DateParserError> {
    let sanitized_input: String = input
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    match sanitized_input {
        sanitized_input if match_twelve_clock_time(&sanitized_input) => {
            parse_twelve_clock_time(&sanitized_input)
        }
        _ => Err(DateParserError {
            message: format!("{} is not a valid datetime", input),
        }),
    }
}

fn parse_twelve_clock_time(input: &str) -> Result<DateTime<Local>, DateParserError> {
    let mut sanitized_input = input.to_string();

    if !input.contains(":") {
        sanitized_input.insert_str(input.len() - 2, ":00");
    }
    let time =
        NaiveTime::parse_from_str(&sanitized_input, "%I:%M%p").map_err(|_| DateParserError {
            message: format!("{} is not a valid time", input),
        })?;
    let local = Local::now();

    local.with_time(time).single().ok_or(DateParserError {
        message: format!("{} is not a valid time", input),
    })
}

fn match_twelve_clock_time(input: &str) -> bool {
    let re = regex::Regex::new(r"^(0?[1-9]|1[0-2])(:[0-5]?[0-9])?(am|pm)$").unwrap();

    re.is_match(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_twelve_clock_time() {
        let examples = vec![
            ("12:00am", 0, 0),
            ("12:00pm", 12, 0),
            ("1:00am", 1, 0),
            ("1:00pm", 13, 0),
            ("1:30am", 1, 30),
            ("1:30pm", 13, 30),
            ("01:30am", 1, 30),
            ("01:30pm", 13, 30),
        ];

        for (input, hour, minute) in examples {
            let result = parse_date(input);
            assert!(result.is_ok());

            let datetime = result.unwrap();

            assert_eq!(datetime.hour(), hour);
            assert_eq!(datetime.minute(), minute);
            assert_eq!(datetime.second(), 0);

            assert_eq!(datetime.day(), Local::now().day());
            assert_eq!(datetime.month(), Local::now().month());
            assert_eq!(datetime.year(), Local::now().year());
        }
    }

    #[test]
    fn test_parse_date() {
        let examples = vec![
            ("12:00am", 0, 0),
            ("12:00pm", 12, 0),
            ("1:00am", 1, 0),
            ("1:00pm", 13, 0),
            ("1:30am", 1, 30),
            ("1:30pm", 13, 30),
            ("01:30am", 1, 30),
            ("01:30pm", 13, 30),
            ("1am", 1, 0),
            ("1pm", 13, 0),
            ("1:30am", 1, 30),
            ("1:30pm", 13, 30),
            ("01:30am", 1, 30),
            ("01:30pm", 13, 30),
            ("12:00 am", 0, 0),
            ("12:00 pm", 12, 0),
            ("1:00 am", 1, 0),
            ("1:00 pm", 13, 0),
            ("1:30 am", 1, 30),
            ("1:30 pm", 13, 30),
            ("01:30 am", 1, 30),
            ("01:30 pm", 13, 30),
            ("1 am", 1, 0),
            ("1 pm", 13, 0),
            ("1:30 am", 1, 30),
            ("1:30 pm", 13, 30),
            ("01:30 am", 1, 30),
            ("01:30 pm", 13, 30),
        ];

        for (input, hour, minute) in examples {
            let result = parse_date(input);
            assert!(result.is_ok());

            let datetime = result.unwrap();

            assert_eq!(datetime.hour(), hour);
            assert_eq!(datetime.minute(), minute);
            assert_eq!(datetime.second(), 0);

            assert_eq!(datetime.day(), Local::now().day());
            assert_eq!(datetime.month(), Local::now().month());
            assert_eq!(datetime.year(), Local::now().year());
        }
    }
}
