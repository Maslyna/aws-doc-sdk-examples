/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_qldbsession::types::StartSessionRequest;
use aws_sdk_qldbsession::{config::Region, meta::PKG_VERSION, Client, Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the ledger.
    #[structopt(short, long)]
    ledger: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Starts a session.
// snippet-start:[qldb.rust.qldb-helloworld]
async fn start(client: &Client, ledger: &str) -> Result<(), Error> {
    let result = client
        .send_command()
        .start_session(StartSessionRequest::builder().ledger_name(ledger).build())
        .send()
        .await?;

    println!(
        "Session id: {:?}",
        result.start_session().unwrap().session_token()
    );

    Ok(())
}
// snippet-end:[qldb.rust.qldb-helloworld]

/// Creates a low-level Amazon Quantum Ledger Database (Amazon QLDB) session in the Region.
/// # Arguments
///
/// * `-l LEDGER` - The name of the ledger to start a new session against.
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        ledger,
        region,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();

    if verbose {
        println!("OLDB client version: {}", PKG_VERSION);
        println!(
            "Region:              {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Ledger:              {}", ledger);
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    start(&client, &ledger).await
}
