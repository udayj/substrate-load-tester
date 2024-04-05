// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(
    runtime_metadata_path = "metadata.scale",
    derive_for_all_types = "Clone, Debug, Eq, PartialEq",
    derive_for_type(path = "primitive_types::U256", derive = "Hash")
)]
pub mod polkadot {}
