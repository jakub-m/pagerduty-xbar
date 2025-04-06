use chrono::{DateTime, Duration, Local, SecondsFormat, TimeDelta, Utc};
use clap::Parser;
use reqwest::Client;
use tracing::{Level, debug};
use types::{Oncall, Oncalls};

mod types;

const GREEN_CIRCLE: char = char::from_u32(0x1F7E2).unwrap();
const YELLOW_CIRCLE: char = char::from_u32(0x1F7E1).unwrap();
const CENTER_X: char = char::from_u32(0x00D7).unwrap();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let log_level = if args.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(log_level)
        .init();

    let now = Utc::now();
    debug!("now={}", &now.to_rfc3339());
    let oncalls = get_oncalls(
        now,
        now + Duration::days(14),
        &args.auth_token,
        &args.schedule_id,
        &args.user_id,
    )
    .await?;

    // Is there any oncall that already started and did not yet finished?
    let is_oncall_now = oncalls.iter().any(|o| now >= o.start && now < o.end);
    debug!("is_oncall_now={is_oncall_now}");
    // Is there any oncall that starts between now and now+DT?
    //let is_oncall_next_24h = is_oncall(now + Duration::days(1), &oncalls);
    // ---------now----------now+1d----------
    // --start------------------------------- no
    // --------------start------------------- yes
    // -----------------------------start---- no
    let is_oncall_next_24h = oncalls
        .iter()
        .any(|o| o.start >= now && o.start <= now + Duration::days(1));
    debug!("is_oncall_next_24h={is_oncall_next_24h}");

    let (icon_on_duty, icon_incoming_on_duty, icon_not_on_duty) = get_icons(&args.icons);

    if is_oncall_now {
        println!("{icon_on_duty}");
    } else if is_oncall_next_24h {
        println!("{icon_incoming_on_duty}");
    } else {
        println!("{icon_not_on_duty}");
    }

    println!("---");
    println!("Upcoming:");

    for oncall in &oncalls {
        let delta = oncall.start - now;

        let time_remaining = if delta <= TimeDelta::zero() {
            format!("now, {}", format_timedelta(oncall.end - now))
        } else {
            format_timedelta(delta)
        };

        println!(
            "{} - ({}) {} - {}",
            oncall.schedule.summary,
            time_remaining,
            format_readable(oncall.start),
            format_readable(oncall.end)
        );
    }

    Ok(())
}

fn format_readable(dt: DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.into();
    local.format("%a %d %b %H:%M").to_string()
}

fn format_iso(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Secs, true)
}

/// https://developer.pagerduty.com/api-reference/3a6b910f11050-list-all-of-the-on-calls
async fn get_oncalls(
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    pg_auth_token: &str,
    pg_schedule_id: &str,
    pg_user_id: &str,
) -> Result<Vec<Oncall>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let since = format_iso(from);
    let until = format_iso(to);
    debug!("pg_schedule_id={pg_schedule_id}");
    debug!("pg_user_id={pg_user_id}");
    debug!("since={since}");
    debug!("until={until}");
    let res = client
        .get("https://api.pagerduty.com/oncalls")
        .query(&[
            ("schedule_ids[]", pg_schedule_id),
            ("since", &since),
            ("until", &until),
        ])
        .header("Authorization", format!("Token token={pg_auth_token}"))
        .send()
        .await?;

    let oncalls: Oncalls = res.json().await?;
    debug!("Got oncalls={oncalls:?}");

    let user_oncalls = oncalls
        .oncalls
        .into_iter()
        .filter(|o| o.user.id == pg_user_id)
        .collect();

    debug!("User oncalls={user_oncalls:?}");
    Ok(user_oncalls)
}

fn get_icons(icons: &str) -> (char, char, char) {
    let icons: Vec<char> = icons.chars().collect();
    (
        *icons.first().unwrap_or(&GREEN_CIRCLE),
        *icons.get(1).unwrap_or(&YELLOW_CIRCLE),
        *icons.get(2).unwrap_or(&CENTER_X),
    )
}

fn format_timedelta(delta: TimeDelta) -> String {
    if delta < TimeDelta::hours(1) {
        format!("{}m", delta.num_minutes())
    } else if delta < TimeDelta::days(1) {
        format!("{}h", delta.num_hours())
    } else {
        format!("{}d", delta.num_days())
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    #[arg(long, env = "PG_SCHEDULE_ID")]
    schedule_id: String,
    #[arg(long, env = "PG_AUTH_TOKEN")]
    auth_token: String,
    #[arg(long, env = "PG_USER_ID")]
    user_id: String,
    #[arg(long, env = "PG_ICONS", default_value_t = String::from(""))]
    icons: String,
}
