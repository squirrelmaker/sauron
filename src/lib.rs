//#![deny(warnings)]
#![deny(clippy::all)]
#![feature(arbitrary_self_types)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png"
)]

//!
//! [![Latest Version](https://img.shields.io/crates/v/sauron.svg)](https://crates.io/crates/sauron)
//! [![Build Status](https://travis-ci.org/ivanceras/sauron.svg?branch=master)](https://travis-ci.org/ivanceras/sauron)
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! ![sauron](https://raw.githubusercontent.com/ivanceras/sauron/master/assets/sauron.png)
//!
//!
//! > One crate to rule the DOM
//! >
//! > One crate to mind it
//! >
//! > One crate to bring JSON
//! >
//! > And in the Rust code bind it
//! >
//! >
//! >
//! > This code, no other, is made by code elves
//! >
//! > Who'd pawn parent process to get it themselves
//! >
//! > Ruler of net troll and mortal and hacker
//! >
//! > This code is a lib crate for Patreon backers
//! >
//! > If trashed or buggy it cannot be remade
//! >
//! > If found send to Ivan, the bandwidth is prepaid
//!
//! -- The Harvard Lampoon & [po8](https://www.reddit.com/user/po8/)
//!
//!
//!  **Sauron** is an html web framework for building web-apps with the goal of
//!  closely adhering to [The Elm Architecture](https://guide.elm-lang.org/architecture/), a paragon for elegant design.
//!
//!  As with elm, sauron follows the simplistic design of writing view code.
//!
//! ### Example
//! ```rust,no_run
//! use sauron::html::attributes::*;
//! use sauron::html::events::*;
//! use sauron::*;
//! use sauron::Component;
//! use sauron::Node;
//! use sauron::Program;
//! use sauron::Cmd;
//! use wasm_bindgen::prelude::*;
//! use log::*;
//!
//! #[derive(Debug, PartialEq, Clone)]
//! pub enum Msg {
//!     Click,
//! }
//!
//! pub struct App {
//!     click_count: u32,
//! }
//!
//! impl App {
//!     pub fn new() -> Self {
//!         App { click_count: 0 }
//!     }
//! }
//!
//! impl Component<Msg> for App {
//!
//!     fn view(&self) -> Node<Msg> {
//!         div!(
//!             [class("some-class"), id("some-id"), attr("data-id", 1)],
//!             [
//!                 input!(
//!                     [
//!                         class("client"),
//!                         type_("button"),
//!                         value("Click me!"),
//!                         onclick(|_| {
//!                             trace!("Button is clicked");
//!                             Msg::Click
//!                         }),
//!                     ],
//!                     [],
//!                 ),
//!                 text!("Clicked: {}", self.click_count),
//!             ],
//!         )
//!     }
//!
//!     fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
//!         trace!("App is updating from msg: {:?}", msg);
//!         match msg {
//!             Msg::Click => {
//!                 self.click_count += 1;
//!                 Cmd::none()
//!             }
//!         }
//!     }
//!
//! }
//!
//! #[wasm_bindgen(start)]
//! pub fn main() {
//!     Program::mount_to_body(App::new());
//! }
//! ```
//! index.html
//! ```html
//! <html>
//!   <head>
//!     <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
//!     <title>Minimal sauron app</title>
//!   </head>
//!   <body>
//!     <script src='pkg/minimal.js'></script>
//!     <script type=module>
//!         window.wasm_bindgen('pkg/minimal_bg.wasm')
//!             .catch(console.error);
//!     </script>
//!   </body>
//! </html>
//! ```
//!
//! **Note:** You need to use the nightly compiler with minimum version: rustc 1.37.0-nightly (17e62f77f 2019-07-01)
//!
//! Build using
//! ```sh
//! $> wasm-pack build --target no-modules
//! ```
//! Look at the [examples](https://github.com/ivanceras/sauron/tree/master/examples)
//! and the build script for the details.
//!
//!
//! ### Prerequisite:
//!
//! ```sh
//! cargo install wasm-pack
//! cargo install basic-http-server
//! ```
//!
//! **Warning:** I repeat, You need to use the latest nightly compiler in order for this to work.
//!
//! This project is based on the existing projects:
//!  - [percy](https://github.com/chinedufn/percy)
//!  - [yew](https://github.com/DenisKolodin/yew)
//!  - [willow](https://github.com/sindreij/willow)
//!
//! ### Performance:
//! ![Benchmark](https://raw.githubusercontent.com/ivanceras/todomvc-perf-comparison/sauron-benchmark/sauron-0.10.0.png)
//!
//! ### Please support this project:
//!  [![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/ivanceras)
//!
//!
//!

use cfg_if::cfg_if;

cfg_if! {if #[cfg(feature = "with-dom")] {
    pub mod dom;
    pub use dom::*;

    use wasm_bindgen::{
        JsCast,
        JsValue,
    };

    /// This needs wrapping only so that we can implement
    /// PartialEq for testing purposes
    #[derive(Clone, Debug)]
    pub struct DomEvent(pub web_sys::Event);

    impl std::ops::Deref for crate::Event {
        type Target = web_sys::Event;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl PartialEq for crate::Event {
        fn eq(&self, other: &Self) -> bool {
            let js_value: Option<&JsValue> = self.0.dyn_ref();
            let other_value: Option<&JsValue> = other.0.dyn_ref();
            js_value == other_value
        }
    }
}}

#[cfg(feature = "with-dom")]
pub type Event = DomEvent;

#[cfg(not(feature = "with-dom"))]
pub type Event = ();

cfg_if! {if #[cfg(feature = "with-markdown")] {
    mod markdown;
    pub use markdown::*;
}}

#[macro_use]
pub mod html;

#[macro_use]
pub mod svg;

pub use sauron_vdom::{
    diff,
    Callback,
    Dispatch,
    Text,
};

/// A simplified version of saurdon_vdom node, where we supplied the type for the tag
/// which is a &'static str. The missing type is now only MSG which will be supplied by the users
/// App code.
pub type Node<MSG> = sauron_vdom::Node<&'static str, Event, MSG>;
pub type Element<MSG> = sauron_vdom::Element<&'static str, Event, MSG>;
pub type Patch<'a, MSG> = sauron_vdom::Patch<'a, &'static str, Event, MSG>;
pub type Attribute<MSG> = sauron_vdom::Attribute<Event, MSG>;
