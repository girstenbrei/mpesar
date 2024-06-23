use async_nats::jetstream::stream::{Config, DiscardPolicy};
use miette::{Context, IntoDiagnostic, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the NATS server
    let client = async_nats::connect("localhost")
        .await
        .into_diagnostic()
        .wrap_err("Failed connecting")?;
    // Create a JetStream context.
    let jetstream = async_nats::jetstream::new(client);

    let _stream = jetstream
        .create_stream(Config {
            name: "events".to_string(),
            max_messages: 100_000,
            discard: DiscardPolicy::Old,
            ..Default::default()
        })
        .await
        .into_diagnostic()
        .wrap_err("Failed creating stream")?;

    // Publish JetStream messages, manage streams, consumers, etc.
    let publish = jetstream
        .publish("events", "bar".into())
        .await
        .into_diagnostic()
        .wrap_err("Failed publishing")?
        .await
        .into_diagnostic()
        .wrap_err("Failed ack for publish")?;

    println!("Published #{} to {}", publish.sequence, publish.stream);

    // // Get or create a pull-based consumer
    // let consumer = stream
    //     .get_or_create_consumer(
    //         "consumer",
    //         async_nats::jetstream::consumer::pull::Config {
    //             durable_name: Some("consumer".to_string()),
    //             ..Default::default()
    //         },
    //     )
    //     .await
    //     .into_diagnostic()
    //     .wrap_err("Failed creating consumer")?;

    // // Consume messages from the consumer
    // let mut messages = consumer
    //     .messages()
    //     .await
    //     .into_diagnostic()
    //     .wrap_err("Failed retrieving messages")?
    //     .take(100);
    // while let Ok(Some(message)) = messages.try_next().await {
    //     println!("message receiver: {:?}", message);
    //     message
    //         .ack()
    //         .await
    //         .map_err(|e| miette!("{}", e))?;
    // }

    println!("Done");
    Ok(())
}
