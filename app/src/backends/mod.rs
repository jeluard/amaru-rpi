#[cfg(feature = "display_hat")]
pub mod display_hat;
#[cfg(feature = "simulator")]
pub mod simulator;

#[cfg(not(any(feature = "simulator", feature = "display_hat")))]
compile_error!("You must enable exactly one of: simulator or display_hat.");
