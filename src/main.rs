#![windows_subsystem = "windows"]

use net_sim::run;

fn main() -> Result<(), impl std::error::Error> {
    pollster::block_on(run())
}
