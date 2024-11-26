pub trait News {
    fn read(self) -> Self;
    fn comments() -> String;
}
