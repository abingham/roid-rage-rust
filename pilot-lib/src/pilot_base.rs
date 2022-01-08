//! Boilerplate for common pilot implementations
use roid_rage_grpc::roid_rage as rpc;
use roid_rage_grpc::roid_rage::pilot_server::{Pilot, PilotServer};
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

/// Implementation of main() for Pilots.
///
/// This main implementation will give you a program that accepts
/// arguments for the pilot address and the game address.
/// 
/// # Example
/// 
/// ```ignore
/// use roid_rage_grpc::roid_rage::pilot_server::Pilot;
/// 
/// struct MyPilot {}
/// 
/// impl Pilot for MyPilot {
/// 	async fn get_command(
/// 		&self,
/// 		request: Request<rpc::GameState>,
/// 	) -> Result<Response<rpc::Command>, Status> {
/// 		rpc::Command::null()
/// 	}
/// }
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error:Error>> {
/// 	let pilot = MyPilot { . . .};
/// 	pilot_lib::pilot_base::pilot_main(pilot).await
/// }
/// ```
pub async fn pilot_main<P>(pilot: P) -> Result<(), Box<dyn std::error::Error>>
where
	P: Pilot,
{
	let opt = Opt::from_args();

	let pilot_address = opt.pilot_address;
	let pilot_url = format!("http://{}", pilot_address);

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

	let _result = handle.await?;
	Ok(())
}
