﻿// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier:  Apache-2.0

// snippet-start:[EventBridge.dotnetv3.HelloEventBridge]
using Microsoft.Extensions.Hosting;
using Amazon.EventBridge;
using Amazon.EventBridge.Model;
using Amazon.Extensions.NETCore.Setup;
using Microsoft.Extensions.DependencyInjection;

namespace EventBridgeActions;

public static class HelloEventBridge
{
    static async Task Main(string[] args)
    {
        // Use the AWS .NET Core Setup package to set up dependency injection for the Amazon EventBridge service.
        // Use your AWS profile name, or leave it blank to use the default profile.
        using var host = Host.CreateDefaultBuilder(args)
            .ConfigureServices((_, services) =>
                services.AddAWSService<IAmazonEventBridge>(new AWSOptions() { Profile = "default" })
            ).Build();

        // Now the client is available for injection.
        var eventBridgeClient = host.Services.GetRequiredService<IAmazonEventBridge>();

        Console.WriteLine($"Hello Amazon EventBridge! Following are some of your EventBuses:");
        Console.WriteLine();

        // You can use await and any of the async methods to get a response.
        // Let's get the first five event buses.
        var response = await eventBridgeClient.ListEventBusesAsync(
            new ListEventBusesRequest()
            {
                Limit = 5
            });

        foreach (var eventBus in response.EventBuses)
        {
            Console.WriteLine($"\tEventBus: {eventBus.Name}");
            Console.WriteLine($"\tArn: {eventBus.Arn}");
            Console.WriteLine($"\tPolicy: {eventBus.Policy}");
            Console.WriteLine();
        }
    }
}
// snippet-end:[EventBridge.dotnetv3.HelloEventBridge]