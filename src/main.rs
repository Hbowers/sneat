use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

// Game
pub mod sneat;

// ECS
pub mod components;
pub mod systems;

pub use sneat::Sneat;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    // join keeps it platform agnostic
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let transform_bundle = TransformBundle::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(transform_bundle)?
        .with_bundle(render_bundle)?
        .with(systems::SneatlingMovementSystem, "sneatling_system", &["input_system"]) // this goes (system, name for system, depends on systems)
        ;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Sneat, game_data)?;
    game.run();

    Ok(())
}
