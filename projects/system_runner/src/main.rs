use bevy::{prelude::*, winit::WinitSettings};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

/// 帧数
#[derive(Resource)]
struct FrameNumber(usize);

/// 固定帧数
#[derive(Resource)]
struct FixedFrameNumber(usize);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // 设置资源路径
            file_path: "../../assets".to_string(),
            ..default()
        }))
        // .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .insert_resource(FrameNumber(0))
        .insert_resource(FixedFrameNumber(0))
        .add_systems(PreStartup, || {
            info!("启动");
        })
        .add_systems(Startup, || {
            info!("初始化");
        })
        .add_systems(PostStartup, || {
            info!("启动后");
        })
        .add_systems(First, |mut frame: ResMut<FrameNumber>| {
            frame.0 += 1;
            info!("帧 {:<4} 开始", frame.0);
        })
        // .add_systems(PreUpdate, |frame: Res<FrameNumber>| {
        //     info!("帧 {:<4} 更新前", frame.0);
        // })
        .add_systems(Update, |frame: Res<FrameNumber>| {
            info!("帧 {:<4} 更新", frame.0);
        })
        .add_systems(SpawnScene, |frame: Res<FrameNumber>| {
            info!("帧 {:<4} 生成场景", frame.0);
        })
        // .add_systems(PostUpdate, |frame: Res<FrameNumber>| {
        //     info!("帧 {:<4} 更新后", frame.0);
        // })
        .add_systems(Last, |frame: Res<FrameNumber>| {
            info!("帧 {:<4} 结束", frame.0);
        })
        .add_systems(FixedFirst, |mut frame: ResMut<FixedFrameNumber>| {
            frame.0 += 1;
            info!("固定帧 {:<4} 开始", frame.0);
        })
        // .add_systems(FixedPreUpdate, |frame: Res<FixedFrameNumber>| {
        //     info!("固定帧 {:<4} 更新前", frame.0);
        // })
        .add_systems(FixedUpdate, |frame: Res<FixedFrameNumber>| {
            info!("固定帧 {:<4} 更新", frame.0);
        })
        // .add_systems(FixedPostUpdate, |frame: Res<FixedFrameNumber>| {
        //     info!("固定帧 {:<4} 更新后", frame.0);
        // })
        .add_systems(FixedLast, |frame: Res<FixedFrameNumber>| {
            info!("固定帧 {:<4} 结束", frame.0);
        })
        .add_systems(Startup, setup_ui)
        .add_systems(FixedUpdate, fixed_update)
        .run();
}

///  生成UI
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .observe(change_update_mode)
                .with_child((
                    Text::new("即时模式"),
                    TextFont {
                        font: asset_server.load("fonts/NotoSansCJKsc-VF.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

/// 改变更新模式
fn change_update_mode(
    _trigger: Trigger<Pointer<Click>>,
    desktop_mode: Local<bool>,
    mut commands: Commands,
    mut text: Single<&mut Text>,
) {
    if *desktop_mode {
        info!("即时模式");
        text.0 = "即时模式".to_string();
        commands.insert_resource(WinitSettings::default());
    } else {
        info!("桌面模式");
        text.0 = "桌面模式".to_string();
        commands.insert_resource(WinitSettings::desktop_app());
    }
}

fn fixed_update(mut last_time: Local<f32>, time: Res<Time>, fixed_time: Res<Time<Fixed>>) {
    // Default `Time`is `Time<Fixed>` here
    info!(
        "time since last fixed_update: {}\n",
        time.elapsed_secs() - *last_time
    );

    info!("fixed timestep: {}\n", time.delta_secs());
    // If we want to see the overstep, we need to access `Time<Fixed>` specifically
    info!(
        "time accrued toward next fixed_update: {}\n",
        fixed_time.overstep().as_secs_f32()
    );
    *last_time = time.elapsed_secs();
}
