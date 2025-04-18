#[cfg(feature = "node-local")]
pub static NODE: &str = "http://localhost:8545";

#[cfg(not(feature = "node-local"))]
pub static NODE: &str = "https://ethereum-rpc.publicnode.com";
