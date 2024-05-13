use std::error::Error;
use std::fmt;
use std::io::{self, Write};

use clap::{App, AppSettings, Arg};
use num_cpus;
use rayon::prelude::*;
use wireguard_vanity_lib::{measure_rate, search_for_prefixes};
use x25519_dalek::{PublicKey, StaticSecret};

fn format_time(t: f64) -> String {
    if t > 3600.0 {
        format!("{:.2} hours", t / 3600.0)
    } else if t > 60.0 {
        format!("{:.1} minutes", t / 60.0)
    } else if t > 1.0 {
        format!("{:.1} seconds", t)
    } else if t > 1e-3 {
        format!("{:.1} ms", t * 1e3)
    } else if t > 1e-6 {
        format!("{:.1} us", t * 1e6)
    } else if t > 1e-9 {
        format!("{:.1} ns", t * 1e9)
    } else {
        format!("{:.3} ps", t * 1e12)
    }
}

fn format_rate(rate: f64) -> String {
    if rate > 1e9 {
        format!("{:.2}e9 keys/s", rate / 1e9)
    } else if rate > 1e6 {
        format!("{:.2}e6 keys/s", rate / 1e6)
    } else if rate > 1e3 {
        format!("{:.2}e3 keys/s", rate / 1e3)
    } else if rate > 1e0 {
        format!("{:.2} keys/s", rate)
    } else if rate > 1e-3 {
        format!("{:.2}e-3 keys/s", rate * 1e3)
    } else if rate > 1e-6 {
        format!("{:.2}e-6 keys/s", rate * 1e6)
    } else if rate > 1e-9 {
        format!("{:.2}e-9 keys/s", rate * 1e9)
    } else {
        format!("{:.3}e-12 keys/s", rate * 1e12)
    }
}

fn print(res: (StaticSecret, PublicKey)) -> Result<(), io::Error> {
    let private: StaticSecret = res.0;
    let public: PublicKey = res.1;
    let private_b64 = base64::encode(&private.to_bytes());
    let public_b64 = base64::encode(public.as_bytes());
    writeln!(
        io::stdout(),
        "private {}  public {}",
        &private_b64,
        &public_b64
    )
}

#[derive(Debug)]
struct ParseError(String);
impl Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("wireguard-vanity-address")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.3.1")
        .author("Brian Warner <warner@lothar.com>")
        .about("finds Wireguard keypairs with a given string prefix")
        .arg(
            Arg::with_name("PREFIX")
                .required(true)
                .multiple(true)
                .help("A prefix to search for - multiple can be specified at once"),
        )
        .get_matches();
    let prefixes = matches.values_of("PREFIX").unwrap().collect::<Vec<_>>();

    let max_prefix_len = prefixes.iter().map(|prefix| prefix.len()).max().expect("prefixes is not empty");
    let trials_per_key = 64u64.pow(max_prefix_len as u32);

    eprintln!(
        "searching - for the longest prefix, one of every {} keys should match",
        trials_per_key
    );

    // get_physical() appears to be more accurate: hyperthreading doesn't
    // help us much

    if trials_per_key < 2u64.pow(32) {
        let raw_rate = measure_rate();
        eprintln!(
            "one core runs at {}, CPU cores available: {}",
            format_rate(raw_rate),
            num_cpus::get_physical(),
        );
        let total_rate = raw_rate * (num_cpus::get_physical() as f64) / (trials_per_key as f64);
        let seconds_per_key = 1.0 / total_rate;
        eprintln!(
            "est yield: {} per key, {} (for the longest prefix)",
            format_time(seconds_per_key),
            format_rate(total_rate)
        );
    }

    eprintln!("hit Ctrl-C to stop");

    // 1M trials takes about 10s on my laptop, so let it run for 1000s
    (0..100_000_000)
        .into_par_iter()
        .map(|_| search_for_prefixes(prefixes.as_slice()))
        .try_for_each(print)?;
    Ok(())
}
