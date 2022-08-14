///
///
///
pub trait Game {
    ///
    ///
    ///
    fn init_game() -> Self;

    ///
    ///
    ///
    fn run(&mut self) -> &mut Self;

    ///
    ///
    ///
    fn exit(&mut self) -> &mut Self;
}
