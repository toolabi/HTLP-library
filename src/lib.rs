mod create;
mod setup;
mod solve;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let st = setup::setup();
        let c  = create::create();
        let s = solve::solve();
        println!("{} {} {}", st, c, s);
    }
}
