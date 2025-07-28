#[cfg(feature = "mock")]
mod mock;
#[cfg(not(feature = "mock"))]
mod api;

#[cfg(feature = "mock")]
pub use mock::fetch_chatgpt_response;

#[cfg(not(feature = "mock"))]
pub use api::fetch_chatgpt_response;