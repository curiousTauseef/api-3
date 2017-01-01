// Copyright 2015-2017 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use project::Language;
use zdaemon::ConfigFile;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Config {
    pub author: String,
    pub repository: String,
    pub language: Language,
    pub dependencies: Option<Vec<String>>,
}

impl ConfigFile for Config {}
