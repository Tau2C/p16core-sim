pub mod exec;
pub mod mem;
pub mod p16core;
pub mod regs;

// use std::thread;
use std::time::{Duration, Instant};

#[cfg(feature = "trace")]
use tracing::Level;
#[cfg(feature = "trace")]
use tracing_subscriber::FmtSubscriber;

use crate::p16core::P16Core;

#[cfg(feature = "pprof")]
use pprof::ProfilerGuard;

#[cfg(feature = "flame")]
use flame;

fn main() {
    #[cfg(feature = "pprof")]
    let guard = ProfilerGuard::new(10000).unwrap();
    #[cfg(feature = "trace")]
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    #[cfg(feature = "trace")]
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    const CPU_FREQ_HZ: u64 = 20_000_000;
    let cycle_duration = Duration::from_nanos(1_000_000_000 / CPU_FREQ_HZ);

    let mut next_tick = Instant::now();
    let file = "test/src.X.production.hex";
    let mut p16 = P16Core::new(file);

    let run_start = Instant::now();

    for _ in 0..CPU_FREQ_HZ {
        #[cfg(feature = "flame")]
        flame::start("cycle");

        #[cfg(feature = "flame")]
        flame::start("get_next_op");
        let op = p16.get_next_op();
        #[cfg(feature = "flame")]
        flame::end("get_next_op");

        #[cfg(feature = "flame")]
        flame::start("decode");
        let instruction = P16Core::decode(op);
        #[cfg(feature = "flame")]
        flame::end("decode");

        #[cfg(feature = "flame")]
        flame::start("exec_op");
        p16.exec_op(instruction);
        #[cfg(feature = "flame")]
        flame::end("exec_op");

        #[cfg(feature = "flame")]
        flame::end("cycle");

        next_tick += cycle_duration;
        let now = Instant::now();
        if now < next_tick {
            // thread::sleep(next_tick - now);
        } else {
            next_tick = now;
        }
    }

    println!("{:?}", run_start.elapsed());

    // --- Dump flamegraph if feature enabled ---
    #[cfg(feature = "flame")]
    {
        use std::fs::File;
        let file = File::create("flamegraph.html").unwrap();
        flame::dump_html(file).unwrap();
        println!("Flamegraph written to flamegraph.html");
    }

    // --- Dump pprof SVG if feature enabled ---
    #[cfg(feature = "pprof")]
    {
        use std::fs::File;
        if let Ok(report) = guard.report().build() {
            let file = File::create("flamegraph.svg").unwrap();
            report.flamegraph(file).unwrap();
            println!("pprof flamegraph written to flamegraph.svg");
        } else {
            eprintln!("Failed to generate pprof report");
        }
    }
}
