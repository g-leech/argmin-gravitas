mod archive;
mod check;
mod cli;
mod config;
mod extract;
mod local;
mod measure;
mod notify;
mod paywall;
mod rehost;
mod site;
mod state;
mod ui;
mod viz;

fn main() -> anyhow::Result<()> {
    cli::run()
}
