mod tests;

use core::fmt;
use std::collections::HashSet;
use std::{f32::DIGITS, str::FromStr};

use chrono::{Datelike, FixedOffset, Local, NaiveDate, Timelike, Weekday};
use icu::calendar::DateTime;
use icu::datetime::{options::length, DateTimeFormatter, ZonedDateTimeFormatter};
use icu::locid::locale;
use icu::datetime::options::components;
use icu::datetime::time_zone::{self, TimeZoneFormatterOptions};

enum PaddingStyle {
    NOPADDING,
    SPACE,
    ZERO,
}

enum TextCase {
    OPPOSITE,
    SAME,
    CAPITAL,
}

enum Output {
    NumericOutput {
        value: i64,
        padding_width: usize,
        padding_style: PaddingStyle,
    },
    TextOutput {
        value: String,
        text_case: TextCase,
        padding_width: usize,
        padding_style: PaddingStyle,
    },
}

impl Default for Output {
    fn default() -> Self {
        Output::NumericOutput {
            value: 0,
            padding_width: 0,
            padding_style: PaddingStyle::NOPADDING,
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::NumericOutput {
                value,
                padding_width,
                padding_style,
            } => {
                let format_output = match padding_style {
                    PaddingStyle::SPACE => format!("{:>width$}", value, width = *padding_width),
                    PaddingStyle::ZERO => format!("{:0>width$}", value, width = *padding_width),
                    PaddingStyle::NOPADDING => format!("{}", value),
                };
                write!(f, "{}", format_output)
            }
            Output::TextOutput { value, text_case,padding_width,padding_style } => {
                let value = match text_case {
                    TextCase::CAPITAL => value.to_uppercase(),
                    TextCase::OPPOSITE => value.to_lowercase(),
                    TextCase::SAME => value.to_string(),
                };
                let format_output = match padding_style {
                    PaddingStyle::SPACE => format!("{:>width$}", value, width = *padding_width),
                    PaddingStyle::ZERO => format!("{:0>width$}", value, width = *padding_width),
                    PaddingStyle::NOPADDING => format!("{}", value),
                };
                
                write!(f, "{}", format_output)
            }
        }
    }
}

fn main() {
    let datetime = Local::now().with_timezone(Local::now().offset());
    let fmt_string = "%z";
    

    let result = format(fmt_string, datetime);

    //let result = format(fmt_string);

    println!("{}", result);
}

fn format(fmt_string: &str, datetime: chrono::DateTime<FixedOffset>) -> String {
    let mut result: String = "".to_string();
    let mut chars = fmt_string.clone().chars();
    let section_list = partition_fmtstr_into_sections(chars);
    //println!("{:?}", section_list);
    for section in section_list {
        result = result + &format_section(&section, datetime);
    }
    result
}

fn format_section(fmt_string: &str, datetime: chrono::DateTime<FixedOffset>) -> String {
    let locale = locale!("en_US");
    let mut chars = fmt_string.clone().chars().peekable();
    let mut specifier: Output;
    let mut text_case: TextCase = TextCase::SAME;
    let mut padding_style: Option<PaddingStyle> = None;
    let mut padding_wdth: Option<usize>;
    let format_specifiers = HashSet::from(['a', 'A', 'b', 'B', 'C', 'd', 'D', 'e', 'q', 'g', 'G', 'h', 'H', 'I', 'j', 'k', 'l', 'm', 'n', 'N', 'p', 'P', 'q', 'r', 'R', 's', 'S', 't', 'T', 'u', 'U', 'V', 'w', 'W', 'x', 'X', 'y', 'Y', 'z', '%']);
    let format_modifiers = HashSet::from(['#', '-', '_', '^', '+', '0']);
    let mut padding_width_string = "".to_string();
    let naive_date = NaiveDate::from_ymd_opt(datetime.year(), datetime.month(), datetime.day()).unwrap();
    let date = DateTime::try_new_iso_datetime(datetime.year(), datetime.month().try_into().unwrap(), datetime.day().try_into().unwrap(), datetime.hour().try_into().unwrap(), datetime.minute().try_into().unwrap(), datetime.second().try_into().unwrap()).unwrap();
    let date = date.to_any();
    let mut result: String = "".into();
    let mut output: String;

    chars.next();
    while let Some(current_char) = chars.next_if(|&c| format_modifiers.contains(&c)) {
        match current_char {
            '#' => text_case = TextCase::OPPOSITE,
            '-' => padding_style = Some(PaddingStyle::NOPADDING),
            '_' => padding_style = Some(PaddingStyle::SPACE),
            '^' => text_case = TextCase::CAPITAL,
            '0' => padding_style = Some(PaddingStyle::ZERO),
            //TODO implement format modifier '+'
            _ => (),
        }
    }

    while let Some(current_char) = chars.next_if(|&c| c.is_ascii_digit()) {
        padding_width_string = padding_width_string + &current_char.to_string();
    }

    if let Some(current_char) = chars.next_if(|&c| format_specifiers.contains(&c)) {
        
        result = result
            + match current_char {
                '%' => {
                    specifier = Output::TextOutput {
                        value: "%".to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: TextCase::SAME,
                    };
                    output = specifier.to_string();
                    &output
                }
                'b' => {
                    let mut bag = components::Bag::default();
                    bag.month = Some(components::Month::Short);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf = DateTimeFormatter::try_new_experimental(
                        &locale.into(),
                        options,
                    )
                    .unwrap();
                    specifier = Output::TextOutput {
                        value: dtf.format(&date).unwrap().to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'B' => {
                    let mut bag = components::Bag::default();
                    bag.month = Some(components::Month::Long);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf = DateTimeFormatter::try_new_experimental(
                        &locale.into(),
                        options,
                    )
                    .unwrap();
                    specifier = Output::TextOutput {
                        value: dtf.format(&date).unwrap().to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'a' => {
                    let mut bag = components::Bag::default();
                    bag.weekday = Some(components::Text::Short);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf = DateTimeFormatter::try_new_experimental(
                        &locale.into(),
                        options,
                    )
                    .unwrap();
                    specifier = Output::TextOutput {
                        value: dtf.format(&date).unwrap().to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'A' => {
                    let mut bag = components::Bag::default();
                    bag.weekday = Some(components::Text::Long);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf = DateTimeFormatter::try_new_experimental(
                        &locale.into(),
                        options,
                    )
                    .unwrap();
                    specifier = Output::TextOutput {
                        value: dtf.format(&date).unwrap().to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'C' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.year() / 100).into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'd' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.day()).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'D' => {
                    specifier = Output::TextOutput {
                        value: format("%m/%d/%y", datetime),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case
                    };
                    output = specifier.to_string();
                    &output
                }
                'e' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.day()).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::SPACE),
                    };
                    output = specifier.to_string();
                    &output
                }
                'g' => {
                    specifier = Output::NumericOutput {
                        value: (naive_date.iso_week().year() % 100).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'G' => {
                    specifier = Output::NumericOutput {
                        value: naive_date.iso_week().year().into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'h' => {
                    let mut bag = components::Bag::default();
                    bag.month = Some(components::Month::Short);
                    let options = icu::datetime::DateTimeFormatterOptions::Components(bag);
                    let dtf = DateTimeFormatter::try_new_experimental(
                        &locale.into(),
                        options,
                    )
                    .unwrap();
                    specifier = Output::TextOutput {
                        value: dtf.format(&date).unwrap().to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'H' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.hour()).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'I' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.hour12()).1.into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'j' => {
                    specifier = Output::NumericOutput {
                        value: datetime.ordinal().into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'k' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.hour()).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::SPACE),
                    };
                    output = specifier.to_string();
                    &output
                }
                'l' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.hour12()).1.into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::SPACE),
                    };
                    output = specifier.to_string();
                    &output
                }
                'm' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.month()).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'M' => {
                    specifier = Output::NumericOutput {
                        value: datetime.minute().into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'n' => {
                    specifier = Output::TextOutput {
                        value: "\n".to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: TextCase::SAME,
                    };
                    output = specifier.to_string();
                    &output
                }
                'N' => {
                    specifier = Output::NumericOutput {
                        value: datetime.nanosecond().into(),
                        padding_width: padding_width_string.parse().unwrap_or(9),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'p' => {
                    specifier = Output::TextOutput {
                        value: {
                            match datetime.hour12().0 {
                                false => "AM".to_owned(),
                                true => "PM".to_owned()
                            }
                        },
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'P' => {
                    specifier = Output::TextOutput {
                        value: {
                            match datetime.hour12().0 {
                                false => "am".to_owned(),
                                true => "pm".to_owned()
                            }
                        },
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                'q' => {
                    specifier = Output::NumericOutput {
                        value: ((datetime.month() / 3) + 1).into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'r' => {
                    specifier = Output::TextOutput {
                        value: format("%l:%M:%S %p", datetime),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case
                    };
                    output = specifier.to_string();
                    &output
                }
                'R' => {
                    specifier = Output::TextOutput {
                        value: format("%H:%M", datetime),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case
                    };
                    output = specifier.to_string();
                    &output
                }
                's' => {
                    specifier = Output::NumericOutput {
                        value: datetime.timestamp(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'S' => {
                    specifier = Output::NumericOutput {
                        value: datetime.second().into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                't' => {
                    specifier = Output::TextOutput {
                        value: "\t".to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: TextCase::SAME,
                    };
                    output = specifier.to_string();
                    &output
                }
                'T' => {
                    specifier = Output::TextOutput {
                        value: format("%H:%M:%S", datetime),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case
                    };
                    output = specifier.to_string();
                    &output
                }
                'u' => {
                    specifier = Output::NumericOutput {
                        value: datetime.weekday().number_from_monday().into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'U' => {
                    specifier = Output::NumericOutput {
                        value: (naive_date.week(Weekday::Sun).first_day().ordinal0() / 7 + 1)
                            .into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'V' => {
                    specifier = Output::NumericOutput {
                        value: naive_date.iso_week().week().into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'W' => {
                    specifier = Output::NumericOutput {
                        value: (naive_date.week(Weekday::Mon).first_day().ordinal0() / 7 + 1)
                            .into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'w' => {
                    specifier = Output::NumericOutput {
                        value: datetime.weekday().num_days_from_sunday().into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'x' => {
                    specifier = Output::TextOutput {
                        value: format("%D", datetime),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case
                    };
                    output = specifier.to_string();
                    &output
                }
                'X' => {
                    specifier = Output::TextOutput {
                        value: format("%T", datetime),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case
                    };
                    output = specifier.to_string();
                    &output
                }
                'y' => {
                    specifier = Output::NumericOutput {
                        value: (datetime.year() % 100).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'Y' => {
                    specifier = Output::NumericOutput {
                        value: datetime.year().into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
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
                        Default::default()
                    )
                    .unwrap();
                    specifier = Output::TextOutput {
                        value: dtf.format(&date,&time_zone).unwrap().to_string(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
                        text_case: text_case,
                    };
                    output = specifier.to_string();
                    &output
                }
                _ => {
                    output = "".to_string();
                    &output
                }
            }
    } else {
        specifier = Output::TextOutput {
            value: fmt_string.to_owned(),
            padding_width: padding_width_string.parse().unwrap_or(0),
            padding_style: padding_style.unwrap_or(PaddingStyle::SPACE),
            text_case: TextCase::SAME,
        };
        result = result + &specifier.to_string();
    }

    //add remaining characters to result
    for c in chars {
        result = result + &c.to_string();
    }
    result
}

fn partition_fmtstr_into_sections(mut chars: std::str::Chars) -> Vec<String> {
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