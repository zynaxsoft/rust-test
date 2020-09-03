// #![allow(unused_imports)]
// use color_eyre::{eyre::Report, eyre::WrapErr, Result, Section};
// use log::{debug, info, trace, warn};
// use thiserror::Error;
// 
// mod util;
// fn main() -> Result<()> {
//     color_eyre::install()?;
//     util::setup_logger()?;
// 
//     Ok(())
// }

fn main() {
    let kos = Kos(10);
    let zin = Zin("woah".into());

    let kos_dyn = dispatch_shake(&kos);
    let zin_dyn = dispatch_shake(&zin);

    downcast_shake(kos_dyn);
    downcast_shake(zin_dyn);
}

struct Kos(u8);
struct Zin(String);

trait Shakable {
    fn shake(&self);
}

impl Shakable for Kos {
    fn shake(&self) {
        println!("Kos shake {}.", self.0);
    }
}

impl Shakable for Zin {
    fn shake(&self) {
        println!("Zin shake {}.", self.0);
    }
}

fn dispatch_shake<T>(s: &T) -> &dyn Ultima where T: Ultima {
    s.shake();
    s
}

enum ShakeType {
    Kos,
    Zin,
}

trait Downcast {
    fn downcast(&self) -> ShakeType;
}

trait Ultima: Shakable + Downcast {
}

impl Ultima for Kos {}
impl Ultima for Zin {}

impl Downcast for Kos {
    fn downcast(&self) -> ShakeType {
        ShakeType::Kos
    }
}

impl Downcast for Zin {
    fn downcast(&self) -> ShakeType {
        ShakeType::Zin
    }
}

fn downcast_shake(s: &dyn Ultima) {
    match s.downcast() {
        ShakeType::Kos => println!("Got Kos"),
        ShakeType::Zin => println!("Got Zin"),
    }
}
