//! 显示三种时间的例子
//! - 真实时间
//! - 固定时间
//! - 虚拟时间

use core::time::Duration;

use bevy::{
    color::palettes::css::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::common_conditions::input_just_pressed,
    prelude::*,
    time::common_conditions::on_real_timer,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // 设置资源路径
            file_path: "../../assets".to_string(),
            ..default()
        }))
        .add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()))
        // 设置固定时间为0.25秒
        .insert_resource(Time::<Fixed>::from_seconds(0.25))
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, )
        .add_systems(
            Update,
            (
                move_virtual_time_sprites,
                move_real_time_sprites,
                move_fixed_time_sprites,
                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
                block_system.run_if(input_just_pressed(KeyCode::KeyP)),
                change_time_speed::<1>.run_if(input_just_pressed(KeyCode::ArrowUp)),
                change_time_speed::<-1>.run_if(input_just_pressed(KeyCode::ArrowDown)),
                (
                    update_virtual_time_info_text,
                    update_fixed_time_info_text,
                    update_real_time_info_text,
                )
                    // update the texts on a timer to make them more readable
                    // `on_timer` run condition uses `Virtual` time meaning it's scaled
                    // and would result in the UI updating at different intervals based
                    // on `Time<Virtual>::relative_speed` and `Time<Virtual>::is_paused()`
                    .run_if(on_real_timer(Duration::from_millis(250))),
            ),
        )
        .run();
}

/// `Real` time related marker
#[derive(Component)]
struct RealTime;

/// `Virtual` time related marker
#[derive(Component)]
struct VirtualTime;

#[derive(Component)]
struct FixedTime;

/// Setup the example
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut time: ResMut<Time<Virtual>>) {
    // 设置虚拟时间的相对速度
    time.set_relative_speed(2.);

    commands.spawn(Camera2d);

    let virtual_color = GOLD.into();
    let fixed_color = LIGHT_SKY_BLUE.into();
    let sprite_scale = Vec2::splat(0.5).extend(1.);
    let texture_handle = asset_server.load("branding/icon.png");
    let font_handle = asset_server.load("fonts/NotoSansCJKsc-VF.ttf");

    // 真实时间
    commands.spawn((
        Sprite::from_image(texture_handle.clone()),
        Transform {
            scale: sprite_scale,
            translation: Vec3::new(0., 80., 0.),
            ..default()
        },
        RealTime,
    ));

    // 固定时间
    commands.spawn((
        Sprite {
            image: texture_handle.clone(),
            color: fixed_color,
            ..Default::default()
        },
        Transform {
            scale: sprite_scale,
            translation: Vec3::new(0., -60., 0.),
            ..default()
        },
        FixedTime,
    ));

    // 虚拟时间
    commands.spawn((
        Sprite {
            image: texture_handle,
            color: virtual_color,
            ..Default::default()
        },
        Transform {
            scale: sprite_scale,
            translation: Vec3::new(0., -200., 0.),
            ..default()
        },
        VirtualTime,
    ));

    // 设置UI
    let font_size = 33.;

    commands
        .spawn(Node {
            display: Display::Flex,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.),
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|builder| {
            // 真实时间信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size,
                    font: font_handle.clone(),
                    ..default()
                },
                RealTime,
            ));

            // 固定时间信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size,
                    font: font_handle.clone(),
                    ..default()
                },
                TextColor(fixed_color),
                TextLayout::new_with_justify(JustifyText::Right),
                FixedTime,
            ));

            // 虚拟时间信息
            builder.spawn((
                Text::default(),
                TextFont {
                    font_size,
                    ..default()
                },
                TextColor(virtual_color),
                TextLayout::new_with_justify(JustifyText::Right),
                VirtualTime,
            ));

            // keybindings
            builder.spawn((
                Text::new("CONTROLS\nUn/Pause: Space\nSpeed+: Up\nSpeed-: Down"),
                TextFont {
                    font_size,
                    font: font_handle,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// 移动真实时间图标
fn move_real_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<RealTime>)>,
    // 真实时间不能缩放和暂停
    time: Res<Time<Real>>,
) {
    info!("Update");
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Real` second
        // when the time is scaled the speed is going to change
        // and the sprite will stay still the time is paused
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

/// 移动固定时间图标
fn move_fixed_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<FixedTime>)>,
    time: Res<Time<Fixed>>,
) {
    info!("FixedUpdate overstep {:.5}", time.overstep().as_secs_f32());
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Real` second
        // when the time is scaled the speed is going to change
        // and the sprite will stay still the time is paused

        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

/// 移动虚拟时间图标
fn move_virtual_time_sprites(
    mut sprite_query: Query<&mut Transform, (With<Sprite>, With<VirtualTime>)>,
    // the default `Time` is either `Time<Virtual>` in regular systems
    // or `Time<Fixed>` in fixed timestep systems so `Time::delta()`,
    // `Time::elapsed()` will return the appropriate values either way
    time: Res<Time>,
) {
    for mut transform in sprite_query.iter_mut() {
        // move roughly half the screen in a `Virtual` second
        // when time is scaled using `Time<Virtual>::set_relative_speed` it's going
        // to move at a different pace and the sprite will stay still when time is
        // `Time<Virtual>::is_paused()`
        transform.translation.x = get_sprite_translation_x(time.elapsed_secs());
    }
}

fn get_sprite_translation_x(elapsed: f32) -> f32 {
    ops::sin(elapsed) * 500.
}

/// Update the speed of `Time<Virtual>.` by `DELTA`
fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
    let time_speed = (time.relative_speed() + DELTA as f32)
        .round()
        .clamp(0.25, 5.);

    // set the speed of the virtual time to speed it up or slow it down
    time.set_relative_speed(time_speed);
}

/// pause or resume `Relative` time
fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

/// Update the `Real` time info text
fn update_real_time_info_text(time: Res<Time<Real>>, mut query: Query<&mut Text, With<RealTime>>) {
    for mut text in &mut query {
        **text = format!(
            "真实时间\nElapsed: {:.1}\nDelta: {:.5}\n",
            time.elapsed_secs(),
            time.delta_secs(),
        );
    }
}

/// Update the `Real` time info text
fn update_fixed_time_info_text(
    time: Res<Time<Fixed>>,
    mut query: Query<&mut Text, With<FixedTime>>,
) {
    for mut text in &mut query {
        **text = format!(
            "固定时间\nElapsed: {:.1}\nDelta: {:.5}\nOverstep: {:.5}",
            time.elapsed_secs(),
            time.delta_secs(),
            time.overstep().as_secs_f32()
        );
    }
}

/// Update the `Virtual` time info text
fn update_virtual_time_info_text(
    time: Res<Time<Virtual>>,
    mut query: Query<&mut Text, With<VirtualTime>>,
) {
    for mut text in &mut query {
        **text = format!(
            "虚拟时间\nElapsed: {:.1}\nDelta: {:.5}\nSpeed: {:.2}",
            time.elapsed_secs(),
            time.delta_secs(),
            time.relative_speed()
        );
    }
}

fn block_system() {
    warn!("阻塞3秒");
    std::thread::sleep(Duration::from_secs(3));
}
