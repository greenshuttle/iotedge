mod decode;
mod encode;
mod mqtt_version;
mod packet;

pub use decode::*;
pub use encode::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
