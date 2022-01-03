#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pilot = manual_pilot::pilot::PilotState::new();
    roid_rage::pilot_base::pilot_main(pilot).await
}
