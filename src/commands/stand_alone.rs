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

use super::Chain;
use crate::cli::{CliResult, CommonFlags};
use mychain_core::{default_database_settings, traits::DefaultAuthorityProvider, Builder};
use mychain_polkadot::{PolkadotBlock, PolkadotExec, PolkadotRtApi};
///! The stand-alone command allows to take-over a stand-alone live system.
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct StandAloneCmd {
	#[structopt(name = "release")]
	release: Option<bool>,
}

impl StandAloneCmd {
	pub fn run(&self, common: CommonFlags) -> CliResult {
		// parse config with serde in struct
		let config = super::parse_config(common.config);

		match config.chain {
			Chain::Polkadot => {
				let mut builder =
					Builder::<PolkadotBlock, PolkadotRtApi, PolkadotExec>::new_with_default(
						default_database_settings(common.data),
					);

				builder
					.swap_authorities::<DefaultAuthorityProvider>()
					// TODO: Pass additional transitions from user input here
					.append_transitions(Vec::new())
					.build_block();

				builder.take_over();
			},
			Chain::Kusama => todo!(),
			Chain::Centrifuge => todo!(),
			Chain::Altair => todo!(),
		};

		Ok(())
	}
}
