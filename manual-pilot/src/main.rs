use roid_rage_grpc::roid_rage as rpc;
use roid_rage_grpc::roid_rage::pilot_server::PilotServer;
use structopt::StructOpt;
use tonic::transport::Server;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// The address on which the pilot will listen
    pilot_address: String,

    // An argument of type float, with a default value.
    #[structopt(short, long, default_value = "[::1]:50051", name = "address")]
    server: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // TODO: The address of this pilot should be a command line option or something.
    let pilot_address = opt.pilot_address;
    let pilot_url = format!("http://{}", pilot_address);

    let pilot = manual_pilot::pilot::PilotState::new();
    let svc = PilotServer::new(pilot);

    // Run the pilot service
    let handle = tokio::spawn(
        Server::builder()
            .add_service(svc)
            .serve(pilot_address.parse().unwrap()),
    );

    let server_address = format!("http://{}", opt.server);

    // Register with game
    let mut client =
        rpc::pilot_registrar_client::PilotRegistrarClient::connect(server_address).await?;
    let request = rpc::RegistrationRequest { url: pilot_url };
    client.register(request).await?;

    // dotenv().ok();
    // env_logger::init();

    // info!("Starting simple-pilot");

    handle.await?;
    Ok(())
}
