use clap::Parser;
use tonic::transport::Server;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "echo-server - a simple echo microservice", long_about = None)]
struct ServerCli {
    #[arg(short = 's', long = "server", default_value = "127.0.0.1")]
    server: String,
    #[arg(short = 'p', long = "port", default_value = "50052")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ServerCli::parse();
    let addr = format!("{}:{}", cli.server, cli.port).parse()?;
    let echo = jeffdb::Echo::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(jeffdb::api::echo::echo_service_server::EchoServiceServer::new(echo))
        .serve(addr)
        .await?;

    Ok(())
}
