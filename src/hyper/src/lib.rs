#![allow(clippy::new_without_default)]

pub use crate::connector::SlackClientHyperConnector;
use slack_morphism::SlackClient;

pub mod connector;
pub mod listener;
pub mod scroller_ext;

pub type SlackHyperClient = SlackClient<SlackClientHyperConnector>;

pub use listener::chain_service_routes_fn;
pub use listener::SlackClientEventsHyperListener;
pub use scroller_ext::SlackApiResponseScrollerExt;
