/*********************** GNU General Public License 3.0 ***********************\
|                                                                              |
|  Copyright (C) 2024 Kevin Matthes                                            |
|                                                                              |
|  This program is free software: you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation, either version 3 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License           |
|  along with this program.  If not, see <https://www.gnu.org/licenses/>.      |
|                                                                              |
\******************************************************************************/

//! <!------------------------------------------------------------------------->
//!
//! [ci]:  https://github.com/kevinmatthes/aeruginous-tbr/workflows/ci/badge.svg
//! [crate]:  https://crates.io/crates/aeruginous-tbr
//! [crates-io]:  https://img.shields.io/crates/v/aeruginous-tbr
//! [deps]:  https://deps.rs/repo/github/kevinmatthes/aeruginous-tbr/status.svg
//! [deps-rs]:  https://deps.rs/repo/github/kevinmatthes/aeruginous-tbr
//! [docs]:  https://docs.rs/aeruginous/badge.svg
//! [docs-rs]:  https://docs.rs/aeruginous-tbr
//! [downloads]:  https://img.shields.io/crates/d/aeruginous-tbr
//! [gpl3]:  https://github.com/kevinmatthes/aeruginous-tbr/blob/main/LICENSE
//! [lcns]:  https://img.shields.io/github/license/kevinmatthes/aeruginous-tbr
//! [lst]: https://img.shields.io/github/last-commit/kevinmatthes/aeruginous-tbr
//! [msrv]:  https://img.shields.io/badge/MSRV-1.76.0-brightgreen
//! [release]:  https://github.com/kevinmatthes/aeruginous-tbr/releases/latest
//! [renovate]:  https://img.shields.io/badge/renovate-enabled-brightgreen.svg
//! [repository]:  https://github.com/kevinmatthes/aeruginous-tbr
//! [tag]:  https://img.shields.io/github/v/tag/kevinmatthes/aeruginous-tbr
//!
//! <!------------------------------------------------------------------------->
//!
//! <p align = 'center'>
//! <a href = 'https://github.com/kevinmatthes/aeruginous-rs'>
//! <img
//!   height = '200'
//!   src =
//!     'https://github.com/kevinmatthes/aeruginous-rs/raw/main/aeruginous.svg'
//! />
//! </a>
//! <br/>
//! The Aeruginous Open Source Development Toolbox
//! </p>
//!
//! ## Summary
//!
//! [![][ci]][repository]
//! [![][lst]][repository]
//! [![][lcns]][repository]
//! [![][renovate]][repository]
//! [![][tag]][release]
//! <br>
//! [![][crates-io]][crate]
//! [![][deps]][deps-rs]
//! [![][docs]][docs-rs]
//! [![][downloads]][crate]
//! [![][msrv]][repository]
//!
//! A plugin for the Aeruginous Open Source Development Toolbox to interact
//! with TAR.BR files.
//!
//! 1. [License](#license)
//! 1. [Dependencies](#dependencies)
//!
//! The current code coverage is **<!-- cov -->0.00%<!-- cov -->**.
//!
//! ## License
//!
//! This project's license is **GPL-3.0**.  The whole license text can be found
//! in [`LICENSE`][gpl3] in the repository root.  The brief version is as
//! follows:
//!
//! > Copyright (C) 2024 Kevin Matthes
//! >
//! > This program is free software: you can redistribute it and/or modify
//! > it under the terms of the GNU General Public License as published by
//! > the Free Software Foundation, either version 3 of the License, or
//! > (at your option) any later version.
//! >
//! > This program is distributed in the hope that it will be useful,
//! > but WITHOUT ANY WARRANTY; without even the implied warranty of
//! > MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! > GNU General Public License for more details.
//! >
//! > You should have received a copy of the GNU General Public License
//! > along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! ## Dependencies
//!
//! - [`sysexits`]
//!   [![](https://img.shields.io/crates/l/sysexits)
//!   ](https://github.com/sorairolake/sysexits-rs)
//!
//! <!------------------------------------------------------------------------->

#![doc(
    html_logo_url = "https://github.com/kevinmatthes/aeruginous-rs/raw/main/aeruginous.svg" // #[aeruginous::mercy::0003]
)]
#![deny(
    clippy::all,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::suspicious,
    clippy::style,
    dead_code,
    deprecated,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    unreachable_code,
    unused_assignments,
    unused_imports,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_variables
)]

mod tar_archive;

pub use tar_archive::TarArchive;

/// This crate's name.
pub const NAME: &str = "aeruginous-tbr";

/// This crate's version.
pub const VERSION: &str = "v0.1.0";

/******************************************************************************/
