mod tests;

use core::fmt;
use std::collections::HashSet;
use std::{f32::DIGITS, str::FromStr};

use chrono::{Datelike, Local, NaiveDate, Weekday};

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
    let fmt_string = "%%abcd%020y";
    

    let mut result: String = "".to_string();

    let mut chars = fmt_string.clone().chars();
    let section_list = partition_fmtstr_into_sections(chars);
    println!("{:?}", section_list);
    for section in section_list {
        result = result + &format_section(&section);
    }

    //let result = format(fmt_string);

    println!("{}", result);
}

fn format_section(fmt_string: &str) -> String {
    let mut chars = fmt_string.clone().chars().peekable();
    let mut specifier: Output;
    let mut text_case: TextCase;
    let mut padding_style: Option<PaddingStyle> = Some(PaddingStyle::NOPADDING);
    let mut padding_wdth: Option<usize>;
    let format_specifiers = HashSet::from(['q', 'C', 'g', 'G', 'm', 'V', 'U', 'W', 'y', 'Y', '%']);
    let format_modifiers = HashSet::from(['#', '-', '_', '^', '+', '0']);
    let mut padding_width_string = "".to_string();
    let now = Local::now().with_timezone(Local::now().offset());
    let naive_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day()).unwrap();
    let mut is_specifier = false;
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
                'q' => {
                    specifier = Output::NumericOutput {
                        value: ((now.month() / 3) + 1).into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'C' => {
                    specifier = Output::NumericOutput {
                        value: (now.year() / 100).into(),
                        padding_width: padding_width_string.parse().unwrap_or(0),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
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
                'm' => {
                    specifier = Output::NumericOutput {
                        value: (now.month()).into(),
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
                'y' => {
                    specifier = Output::NumericOutput {
                        value: (now.year() % 100).into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
                    };
                    output = specifier.to_string();
                    &output
                }
                'Y' => {
                    specifier = Output::NumericOutput {
                        value: now.year().into(),
                        padding_width: padding_width_string.parse().unwrap_or(2),
                        padding_style: padding_style.unwrap_or(PaddingStyle::ZERO),
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
            padding_style: padding_style.unwrap_or(PaddingStyle::NOPADDING),
            text_case: TextCase::SAME,
        };
        result = result + &specifier.to_string();
    }
    while let Some(c) = chars.next() {
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


// fn format(fmt_string: &str) -> String {
//     let datetime = Local::now().with_timezone(Local::now().offset());
//     let naive_date = NaiveDate::from_ymd_opt(datetime.year(), datetime.month(), datetime.day()).unwrap();
//     let mut is_specifier = false;
//     let mut result: String = "".into();
//     let mut output: String;

//     let mut specifier: Output;
//     type ModifierSequence = String;
//     let mut modifier_string: ModifierSequence = "".to_string();

//     //let mut modifier: char = '';

//     let mut is_modifier = false;

//     for char in fmt_string.chars() {
//         if is_specifier || is_modifier {
//             result = result
//                 + match char {
//                     '%' => {
//                         let padding_width: usize = parse_modifier_sequence(&modifier_string).1;
//                         specifier = Output::NumericOutput {
//                             value: ((datetime.month() / 3) + 1).into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'q' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(0);
//                         specifier = Output::NumericOutput {
//                             value: ((datetime.month() / 3) + 1).into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'C' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(0);
//                         specifier = Output::NumericOutput {
//                             value: (datetime.year() / 100).into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'g' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(2);
//                         specifier = Output::NumericOutput {
//                             value: (naive_date.iso_week().year() % 100).into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'G' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(0);
//                         specifier = Output::NumericOutput {
//                             value: naive_date.iso_week().year().into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'm' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(2);
//                         specifier = Output::NumericOutput {
//                             value: (datetime.month()).into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'V' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(2);
//                         specifier = Output::NumericOutput {
//                             value: naive_date.iso_week().week().into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'U' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(2);
//                         specifier = Output::NumericOutput {
//                             value: (naive_date.week(Weekday::Sun).first_day().ordinal0() / 7 + 1)
//                                 .into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'W' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(2);
//                         specifier = Output::NumericOutput {
//                             value: (naive_date.week(Weekday::Mon).first_day().ordinal0() / 7 + 1)
//                                 .into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'y' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(2);
//                         specifier = Output::NumericOutput {
//                             value: (datetime.year() % 100).into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                     'Y' => {
//                         let padding_width: usize = modifier_string.parse().unwrap_or(0);
//                         specifier = Output::NumericOutput {
//                             value: datetime.year().into(),
//                             padding_width: padding_width,
//                             padding_style: PaddingStyle::ZERO,
//                         };
//                         output = specifier.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }

//                     //modifier @ ('#' | '-' | '_' | '^') => {}
//                     modifier_char @ ('1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
//                     | '#' | '-' | '_' | '^') => {
//                         modifier_string = modifier_string + &modifier_char.to_string();
//                         is_modifier = true;
//                         ""
//                     }
//                     _ => {
//                         output = "%".to_string() + &char.to_string();
//                         is_modifier = false;
//                         modifier_string = "".to_string();
//                         &output
//                     }
//                 };
//             is_specifier = false;
//         } else if char == '%' {
//             is_specifier = !is_specifier;
//             is_modifier = false;
//             modifier_string = "".to_string();
//         } else {
//             is_modifier = false;
//             modifier_string = "".to_string();
//             result = result + &char.to_string()
//         }
//     }
//     return result;
// }
// fn parse_modifier_sequence(
//     sequence: &str,
// ) -> (Option<PaddingStyle>, usize, Option<TextCase>, bool) {
//     let mut padding_width_string = "".to_string();
//     let mut text_modifier_string = "".to_string();
//     let mut is_valid = true;
//     for char in sequence.chars() {
//         match char {
//             modifier_char @ ('1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0') => {
//                 padding_width_string = padding_width_string + &modifier_char.to_string();
//             }
//             modifier_char @ ('#' | '-' | '_' | '^') => {
//                 if padding_width_string != "" {
//                     is_valid = false;
//                 } else {
//                     text_modifier_string = text_modifier_string + &modifier_char.to_string();
//                 }
//             }
//             _ => is_valid = false,
//         }
//     }

//     (
//         None,
//         padding_width_string.parse().unwrap_or(0),
//         None,
//         is_valid,
//     )
// }
