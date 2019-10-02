mod sound;

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
        instrument::clarinet();
    }
}


fn main() {
    crate::sound::instrument::clarinet();

    sound::instrument::clarinet();

    crate::sound::hello();

    sound::hello();

    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
}
