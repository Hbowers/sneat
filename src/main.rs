use amethyst::{
    core::{SystemBundle, TransformBundle},
    ecs::DispatcherBuilder,
    error::Error,
    input::{Bindings, InputSystemDesc, StringBindings},
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

pub struct StartingBundle {}

impl<'a, 'b> SystemBundle<'a, 'b> for StartingBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let app_root = application_root_dir()?;

        // join keeps it platform agnostic
        let binding_path = app_root.join("config").join("bindings.ron");

        TransformBundle::default().build(world, builder)?;

        // let input_bundle =
        // InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
        builder.add_thread_local(
            InputSystemDesc::<StringBindings>::new(Some(Bindings::load(binding_path)?))
                .build(world),
        );

        Ok(())
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let render_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(StartingBundle {})?
        .with_bundle(render_bundle)?
        .with(
            systems::SneatlingMovementSystem,
            "sneatling_system",
            &[],
        )
        .with(
            systems::VelocitySystem,
            "velocity_system",
            &["sneatling_system"],
        );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Sneat, game_data)?;
    game.run();

    Ok(())
}
