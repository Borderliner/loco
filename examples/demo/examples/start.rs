use blo::app::App;
use loco_rs::{
    boot::{create_app, start, ServeParams, StartMode},
    environment::{resolve_from_env, Environment},
};
use migration::Migrator;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let environment: Environment = resolve_from_env().into();

    let boot_result = create_app::<App, Migrator>(StartMode::ServerAndWorker, &environment).await?;
    let serve_params = ServeParams {
        port: boot_result.app_context.config.server.port,
        binding: boot_result.app_context.config.server.binding.to_string(),
    };
    start(boot_result, serve_params).await?;
    Ok(())
}
