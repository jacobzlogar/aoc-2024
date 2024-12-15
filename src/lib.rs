use miette::IntoDiagnostic;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

pub mod day12;

pub fn get_input(day: &str) -> miette::Result<String> {
    // this probably shouldn't live here?
    fmt::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_level(false)
        .init();

    let base = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/data/{}.txt", base, day);
    let data = std::fs::read_to_string(&path)
        .map_err(|e| miette::miette!("{e}: {path}"))?;
    Ok(data)
}
