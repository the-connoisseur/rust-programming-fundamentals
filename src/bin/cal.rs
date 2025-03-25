use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;

/// Array of month names for calendar headers.
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "september",
    "October",
    "November",
    "December",
];

#[derive(Parser)]
#[command(version = "1.0.0")]
struct Args {
    /// calendar year [default: current year when the tool is run]
    #[arg()]
    yr: Option<u32>,

    /// month (1 ..= 12) [default: none, prints whole year]
    #[arg(long)]
    month: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let current_year = Local::now().year() as u32;
    let year = args.yr.unwrap_or(current_year);

    if !(1..=9999).contains(&year) {
        eprintln!("[ERROR] Year must be between 1 and 9999");
        return;
    }

    if let Some(month) = args.month {
        if !(1..=12).contains(&month) {
            eprintln!("[ERROR] Month must be between 1 and 12");
            return;
        }
        print_month(year, month);
    } else {
        print_year(year);
    }
}

/// Prints the calendar for a specific month.
fn print_month(year: u32, month: u32) {
    let month_name = MONTH_NAMES[month as usize - 1];
    let header = format!("{} {}", month_name, year);
    // Use a fixed width of 20 and center the text.
    println!("{:^20}", header);
    println!("Mo Tu We Th Fr Sa Su");

    // Calculate the first day and number of days in the month.
    let first_day = NaiveDate::from_ymd_opt(year as i32, month, 1).unwrap();
    let start_weekday = first_day.weekday().num_days_from_monday() as usize;
    let num_days = if month == 12 {
        NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    } else {
        NaiveDate::from_ymd_opt(year as i32, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    };

    // Calculate the number of weeks. Add 6 and divide by 7 to do ceiling division using integer
    // arithmetic.
    let total_positions = start_weekday + num_days as usize;
    let num_weeks = (total_positions + 6) / 7;

    // Generate and print each week's line.
    for week in 0..num_weeks {
        let mut line = Vec::new();
        for weekday in 0..7 {
            let pos = week * 7 + weekday;
            if pos < start_weekday || pos >= start_weekday + num_days as usize {
                line.push("  ".to_string());
            } else {
                let day = (pos - start_weekday + 1) as u32;
                line.push(format!("{:>2}", day));
            }
        }
        println!("{}", line.join(" "));
    }
}

/// Prints the calendar for an entire year.
fn print_year(year: u32) {
    // Use a fixed width of 20 and center the text.
    println!("{:^20}", year);
    println!();

    // Print each month's calendar with a blank line between them.
    for month in 1..=12 {
        print_month(year, month);
        println!();
    }
}
