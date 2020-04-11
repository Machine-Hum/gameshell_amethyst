use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        rendy::hal::command::ClearColor,
        RenderingBundle,
    },
    utils::application_root_dir,
    window::{DisplayConfig, EventLoop},
};

use log::info;

mod state;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config_path = app_root.join("resources/display_config.ron");
    info!("{:?}", display_config_path);
    let display_config = DisplayConfig::load(display_config_path)?;
    
    let event_loop = EventLoop::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new(display_config, &event_loop)
                .with_plugin(RenderToWindow::new().with_clear(ClearColor {
                                   float32: [0.34, 0.36, 0.52, 1.0],
                                                   }))
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(resources, state::MyState, game_data)?;
    game.run();

    Ok(())
}
