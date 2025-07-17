use bevy::prelude::*;

use iyes_perf_ui::prelude::*;

pub struct UIPlugin;
impl bevy::app::Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PerfUiPlugin)
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);
        app.add_systems(Startup, (setup_perfui, setup_substance_selection));
    }
}

fn setup_perfui(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}

fn setup_substance_selection(mut commands: Commands) {
    let container_node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(20.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let button_node = Node {
        width: Val::Px(150.0),
        height: Val::Px(65.0),
        border: UiRect::all(Val::Px(5.0)),
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text = (
        Text::new("Button"),
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        TextFont::default().with_font_size(40.0),
    );

    let button = commands
        .spawn((
            button_node,
            BorderColor(Color::BLACK),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            children![(button_text)],
        ))
        .id();

    commands.spawn(container_node).add_children(&[button]);
}
