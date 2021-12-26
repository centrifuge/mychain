// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

///! Commands of the cli and associated input structs
use serde::{Serialize, Deserialize};
use sp_core::Pair;
use std::path::PathBuf;

pub mod stand_alone;
pub mod parachain;

fn parse_config(path: PathBuf) -> Config {
    todo!();
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    chain: Chain,
    finale: Option<Vec<String>>,
    authorities: Vec<String>,
    transitions: Transitions,
}

#[derive(Debug, Deserialize, Serialize)]
enum Chain {
    Polkadot,
    Kusama,
    Centrifuge,
    Altair
}

#[derive(Debug, Deserialize, Serialize)]
struct Transitions {
    raw: Option<Vec<RawTransitions>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawTransitions{
    key: String,
    value: String,
}