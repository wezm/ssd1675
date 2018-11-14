#![no_std]

extern crate embedded_hal as hal;

mod command;
mod interface;
mod display;

pub use interface::DisplayInterface;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
