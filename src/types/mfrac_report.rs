pub type MfracStatus = Option<MfracOutcome>;

pub enum MfracOutcome {
    Collision(u128),
    Termination(MfracTerminationReason),
}

pub enum MfracTerminationReason {
    LimitExceeded(u64),
    StaleLife,
    Oscillator,
}
