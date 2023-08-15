pub trait Message {
    fn command() -> String where Self: Sized;
    fn parse(stream: Vec<u8>) -> Self where Self: Sized;
    fn serialize(&self) -> Vec<u8>;
}
