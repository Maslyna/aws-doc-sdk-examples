/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{config::Region, meta::PKG_VERSION, Client, Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The ID of the instance to reboot.
    #[structopt(short, long)]
    instance_id: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Reboots an instance.
// snippet-start:[ec2.rust.reboot-instance]
async fn reboot_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.reboot_instances().instance_ids(id).send().await?;

    println!("Rebooted instance.");
    Ok(())
}
// snippet-end:[ec2.rust.reboot-instance]

/// Reboots an Amazon EC2 instance.
/// # Arguments
///
/// * `-i INSTANCE-ID` - The ID of the instances to reboot.
/// * `[-r REGION]` - The Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        region,
        instance_id,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("EC2 client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!("Instance ID:        {}", instance_id);
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    reboot_instance(&client, &instance_id).await
}
