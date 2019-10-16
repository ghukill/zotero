//! # Zotero
//! [![Build Status](https://travis-ci.org/Eonm/zotero.svg?branch=master)](https://travis-ci.org/Eonm/nl80211)
//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Eonm/markdown-packager/issues)
//!
//! [API documentation](https://docs.rs/zotero/)
//!
//! ## Creating items and collections
//!
//! ```no_run
//! extern crate zotero;
//!
//! use zotero::ZoteroInit;
//! use zotero::Post;
//! use zotero::data_structure::item::{BookData, BookDataBuilder, Creator, CreatorBuilder};
//!
//! let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
//!
//! let creators : Vec<Creator> = vec![
//!     CreatorBuilder::default()
//!         .creator_type("author")
//!         .first_name("John")
//!         .last_name("Doe")
//!         .build()
//!         .unwrap()
//! ];
//!
//! let new_book : BookData = BookDataBuilder::default()
//!     .title("Sample_2")
//!     .creators(creators)
//!     .item_type("book")
//!     .build()
//!     .unwrap();
//!
//! z.create_new_item(new_book);
//! ```
//!
//! ## Updating items and collections
//! ```no_run
//! extern crate zotero;
//!
//! use zotero::ZoteroInit;
//! use zotero::Patch;
//! use zotero::Get;
//! use zotero::data_structure::item::ItemType;
//!
//! let z = ZoteroInit::set_user("123456789", "bZARysJ579K5SdmYuaAJ");
//! let item = z.get_item("Q8GNE36F", None);
//! if let Ok(mut result) = item {
//!     if let ItemType::Book(bookdata) = &mut result.data {
//!         bookdata.title = "A new title".to_string();
//!         bookdata.publisher = "A new publisher".to_string();
//!         z.update_item(&bookdata.key, &bookdata);
//!     };
//!
//!     println!("{:?}", serde_json::to_string(&result.data));
//! };
//! ```

mod api_error_parser;
mod api_request;
mod consts;
pub mod data_structure;
mod zotero_api;
pub use api_request::delete::Delete;
pub use api_request::get::Get;
pub use api_request::patch::Patch;
pub use api_request::post::Post;
pub use zotero_api::{Zotero, ZoteroInit};
