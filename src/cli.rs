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
use crate::commands::{parachain::ParachainCmd, stand_alone::StandAloneCmd};
use std::path::PathBuf;
use structopt::StructOpt;

pub use error::Error;

/// The generic return type for the root run call
pub type CliResult = Result<(), Error>;

pub fn run() -> CliResult {
	let args = Cli::from_args();

	// TODO (mustermeiszer): Might have a log flag in the future
	match args.network_type {
		NetworkType::StandAlone { cmd, common } => cmd.run(common),
		NetworkType::Parachain { cmd, common } => cmd.run(common),
	}
}

#[derive(Debug, StructOpt)]
#[structopt(name = "mychain", about = "Take-over live chains to use them locally.")]
pub struct Cli {
	#[structopt(subcommand)]
	network_type: NetworkType,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Network Type")]
pub enum NetworkType {
	#[structopt(name = "stand-alone")]
	StandAlone {
		#[structopt(flatten)]
		cmd: StandAloneCmd,

		#[structopt(flatten)]
		common: CommonFlags,
	},

	#[structopt(name = "parachain")]
	Parachain {
		#[structopt(flatten)]
		cmd: ParachainCmd,

		#[structopt(flatten)]
		common: CommonFlags,
	},
}

#[derive(Debug, StructOpt)]
pub struct CommonFlags {
	#[structopt(short = "d", long = "data", parse(from_os_str))]
	pub data: PathBuf,

	#[structopt(short = "c", long = "config", parse(from_os_str))]
	pub config: PathBuf,

	#[structopt(short = "o", long = "output", parse(from_os_str))]
	pub output: PathBuf,
}

pub mod error {
	#[derive(Debug, thiserror::Error)]
	#[allow(missing_docs)]
	pub enum Error {
		#[error("Invalid input: {0}")]
		Input(String),

		#[error("Command currently not supported {0}")]
		NotSupported(&'static str),
	}
}
