#[macro_use]
extern crate serde_derive;

#[macro_use]
mod builder;
mod core;

pub mod public_api;
pub mod trade_api;

pub use core::AccessKey;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
