pub trait News {
    fn read(self) -> Self;
    fn read_comments() -> String;
}