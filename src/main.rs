mod tests;

use core::fmt;
use std::collections::HashSet;
use chrono::{Datelike, FixedOffset, Local, NaiveDate, Timelike, Weekday};
use icu::calendar::DateTime;
use icu::datetime::options::components;
use icu::datetime::{DateTimeFormatter, ZonedDateTimeFormatter};
use icu::locid::locale;

enum Padding {
    NONE,
    SPACE,
    ZERO,
}

enum Case {
    UPPER,
    OPPOSITE,
    ORIGINAL,
}

enum FormattedOutput {
    Numeric {
        value: i64,
        width: usize,
        padding: Padding,
    },
    Text {
        value: String,
        case: Case,
        width: usize,
        padding: Padding,
    },
}

impl Default for FormattedOutput {
    fn default() -> Self {
        FormattedOutput::Numeric {
            value: 0,
            width: 0,
            padding: Padding::NONE,
        }
    }
}

impl fmt::Display for FormattedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedOutput::Numeric {
                value,
                width,
                padding,
            } => {
                let formatted = match padding {
                    Padding::SPACE => format!("{:>width$}", value, width = *width),
                    Padding::ZERO => format!("{:0>width$}", value, width = *width),
                    Padding::NONE => format!("{}", value),
                };
                write!(f, "{}", formatted)
            }
            FormattedOutput::Text {
                value,
                case,
                width,
                padding,
            } => {
                let value = match case {
                    Case::UPPER => value.to_uppercase(),
                    Case::OPPOSITE => value.to_lowercase(),
                    Case::ORIGINAL => value.to_string(),
                };
                let formatted = match padding {
                    Padding::SPACE => format!("{:>width$}", value, width = *width),
                    Padding::ZERO => format!("{:0>width$}", value, width = *width),
                    Padding::NONE => format!("{}", value),
                };

                write!(f, "{}", formatted)
            }
        }
    }
}

fn main() {
    let current_datetime = Local::now().with_timezone(Local::now().offset());
    let format_string = "%z";

    let formatted_result = format(format_string, current_datetime);

    //let formatted_result = format(format_string);

    println!("{}", formatted_result);
}

fn format(format_string: &str, datetime: chrono::DateTime<FixedOffset>) -> String {
    let mut formatted_result: String = "".to_string();
    
    let section_list = partition_format_string_into_sections(format_string);
    //println!("{:?}", section_list);
    for section in section_list {
        formatted_result = formatted_result + &format_section(&section, datetime);
    }
    formatted_result
}

fn format_section(section_string: &str, datetime: chrono::DateTime<FixedOffset>) -> String {
    let locale = locale!("en_US");
    let mut section_chars = section_string.chars().peekable();
    let formatted_output: FormattedOutput;
    let mut case: Case = Case::ORIGINAL;
    let mut padding: Option<Padding> = None;
    let mut width: Option<usize>;
    let format_specifiers = HashSet::from([
        'a', 'A', 'b', 'B', 'C', 'd', 'D', 'e', 'F', 'q', 'g', 'G', 'h', 'H', 'I', 'j', 'k', 'l', 'm', 'M',
        'n', 'N', 'p', 'P', 'q', 'r', 'R', 's', 'S', 't', 'T', 'u', 'U', 'V', 'w', 'W', 'x', 'X',
        'y', 'Y', 'z', '%',
    ]);
    let format_modifiers = HashSet::from(['#', '-', '_', '^', '+', '0']);
    let mut width_string = "".to_string();
    let naive_date =
        NaiveDate::from_ymd_opt(datetime.year(), datetime.month(), datetime.day()).unwrap();
    let date = DateTime::try_new_iso_datetime(
        datetime.year(),
        datetime.month().try_into().unwrap(),
        datetime.day().try_into().unwrap(),
        datetime.hour().try_into().unwrap(),
        datetime.minute().try_into().unwrap(),
        datetime.second().try_into().unwrap(),
    )
    .unwrap();
    let date = date.to_any();
    let mut formatted_result: String = "".into();
    let output: String;

    section_chars.next();
    while let Some(current_char) = section_chars.next_if(|&c| format_modifiers.contains(&c)) {
        match current_char {
            '#' => case = Case::OPPOSITE,
            '-' => padding = Some(Padding::NONE),
            '_' => padding = Some(Padding::SPACE),
            '^' => case = Case::UPPER,
            '0' => padding = Some(Padding::ZERO),
            //TODO implement format modifier '+'
            _ => (),
        }
    }

    while let Some(current_char) = section_chars.next_if(|&c| c.is_ascii_digit()) {
        width_string = width_string + &current_char.to_string();
    }

    if let Some(current_char) = section_chars.next_if(|&c| format_specifiers.contains(&c)) {
        formatted_result = formatted_result
            + match current_char {
                '%' => {
                    formatted_output = FormattedOutput::Text {
                        value: "%".to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: Case::ORIGINAL,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'b' => {
                    let mut bag = components::Bag::default();
                    bag.month = Some(components::Month::Short);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf =
                        DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();
                    formatted_output = FormattedOutput::Text {
                        value: dtf.format(&date).unwrap().to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'B' => {
                    let mut bag = components::Bag::default();
                    bag.month = Some(components::Month::Long);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf =
                        DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();
                    formatted_output = FormattedOutput::Text {
                        value: dtf.format(&date).unwrap().to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'a' => {
                    let mut bag = components::Bag::default();
                    bag.weekday = Some(components::Text::Short);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf =
                        DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();
                    formatted_output = FormattedOutput::Text {
                        value: dtf.format(&date).unwrap().to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'A' => {
                    let mut bag = components::Bag::default();
                    bag.weekday = Some(components::Text::Long);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf =
                        DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();
                    formatted_output = FormattedOutput::Text {
                        value: dtf.format(&date).unwrap().to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'C' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.year() / 100).into(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'd' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.day()).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'D' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%m/%d/%y", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'e' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.day()).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::SPACE),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'F' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%+4Y-%m-%d", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'g' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (naive_date.iso_week().year() % 100).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'G' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: naive_date.iso_week().year().into(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'h' => {
                    let mut bag = components::Bag::default();
                    bag.month = Some(components::Month::Short);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf =
                        DateTimeFormatter::try_new_experimental(&locale.into(), options).unwrap();
                    formatted_output = FormattedOutput::Text {
                        value: dtf.format(&date).unwrap().to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'H' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.hour()).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'I' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.hour12()).1.into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'j' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.ordinal().into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'k' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.hour()).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::SPACE),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'l' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.hour12()).1.into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::SPACE),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'm' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.month()).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'M' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.minute().into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'n' => {
                    formatted_output = FormattedOutput::Text {
                        value: "\n".to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: Case::ORIGINAL,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'N' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.nanosecond().into(),
                        width: width_string.parse().unwrap_or(9),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'p' => {
                    formatted_output = FormattedOutput::Text {
                        value: {
                            match datetime.hour12().0 {
                                false => "AM".to_owned(),
                                true => "PM".to_owned(),
                            }
                        },
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'P' => {
                    formatted_output = FormattedOutput::Text {
                        value: {
                            match datetime.hour12().0 {
                                false => "am".to_owned(),
                                true => "pm".to_owned(),
                            }
                        },
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'q' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: ((datetime.month() / 3) + 1).into(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'r' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%l:%M:%S %p", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'R' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%H:%M", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                's' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.timestamp(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'S' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.second().into(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                't' => {
                    formatted_output = FormattedOutput::Text {
                        value: "\t".to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: Case::ORIGINAL,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'T' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%H:%M:%S", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'u' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.weekday().number_from_monday().into(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'U' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (naive_date.week(Weekday::Sun).first_day().ordinal0() / 7 + 1)
                            .into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'V' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: naive_date.iso_week().week().into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'W' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (naive_date.week(Weekday::Mon).first_day().ordinal0() / 7 + 1)
                            .into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'w' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.weekday().num_days_from_sunday().into(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'x' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%D", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'X' => {
                    formatted_output = FormattedOutput::Text {
                        value: format("%T", datetime),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'y' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: (datetime.year() % 100).into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'Y' => {
                    formatted_output = FormattedOutput::Numeric {
                        value: datetime.year().into(),
                        width: width_string.parse().unwrap_or(2),
                        padding: padding.unwrap_or(Padding::ZERO),
                    };
                    output = formatted_output.to_string();
                    &output
                }
                'z' => {
                    let mut bag = components::Bag::default();
                    bag.time_zone_name = Some(components::TimeZoneName::LongGeneric);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let time_zone = icu::timezone::CustomTimeZone::utc();
                    let dtf = ZonedDateTimeFormatter::try_new_experimental(
                        &locale.into(),
                        options.into(),
                        Default::default(),
                    )
                    .unwrap();
                    formatted_output = FormattedOutput::Text {
                        value: dtf.format(&date, &time_zone).unwrap().to_string(),
                        width: width_string.parse().unwrap_or(0),
                        padding: padding.unwrap_or(Padding::NONE),
                        case: case,
                    };
                    output = formatted_output.to_string();
                    &output
                }
                _ => {
                    output = "".to_string();
                    &output
                }
            };
        //add remaining characters to result
        for c in section_chars {
            formatted_result = formatted_result + &c.to_string();
        }
    } else {
        formatted_output = FormattedOutput::Text {
            value: section_string.to_owned(),
            width: width_string.parse().unwrap_or(0),
            padding: padding.unwrap_or(Padding::SPACE),
            case: Case::ORIGINAL,
        };
        formatted_result = formatted_result + &formatted_output.to_string();
    }

    formatted_result
}

fn partition_format_string_into_sections(format_string: &str) -> Vec<String> {
    let chars = format_string.chars();
    let mut sections = Vec::new();
    let mut current_section = String::new();

    for c in chars {
        if c == '%' && current_section.len() == 1 {
            current_section.push(c);
        } else if c == '%' && !current_section.is_empty() {
            sections.push(current_section.clone());
            current_section.clear();
            current_section.push(c);
        } else {
            current_section.push(c);
        }
    }

    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}
