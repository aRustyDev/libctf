/// **Pre-built api client calls** for interacting with the CTF platform
///
/// This is primarily used for interacting with the kubernetes API to create custom CTF resources
pub mod api;

/// **Required interfaces code** for extending the CTF platform. (**WIP**)
/// - Challenges: This describes how to get to a 'challenge' similar to a route.
/// - Flags: Flags implement an API interface for generating a flag. This de-couples the way a flag is generated/retrieved from the other logic.
/// - Plugins: Plugins are still a work in progress; they should allow for extending the platform with custom functionality.
/// - Verifiers: Verifiers (similar to flags) implement an API interface for verifying a flag. This de-couples the way a flag is verified from the other logic.
pub mod interface;

/// **Custom resource definitions** for the CTF platform.
///
/// All CRDs are *derived* via the `kube-derive` crate, and the origin code while public is hidden from the docs.
pub mod spec;
