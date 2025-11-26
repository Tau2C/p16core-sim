pub mod exec;
pub mod mem;
pub mod p16core;
pub mod regs;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::p16core::P16Core;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let file = "test/src.X.production.hex";
    let mut p16 = P16Core::new(file);
    for _ in 0..100 {
        let op = p16.get_next_op();
        let instruction = P16Core::decode(op);
        let span = tracing::info_span!("cycle", pc = p16.pc, op);
        let _guard = span.enter();
        tracing::info!("{:?}", instruction);
        p16.exec_op(instruction);
    }
}
