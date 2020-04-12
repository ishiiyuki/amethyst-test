use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::TransformBundle, Transform},
    ecs::prelude::*,
    ecs::System,
    input::{InputBundle, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        types::DefaultBackend,
        RenderingBundle, Texture,
    },
    utils::application_root_dir,
};

/// パラメータ
const SCREEN_WIDTH: f32 = 500.;
const SCREEN_HEIGHT: f32 = 500.;
const OBSTACLE_WIDTH: f32 = 303.;
const OBSTACLE_HEIGHT: f32 = 302.;
const ROCK_HEIGHT: f32 = 52.;
const GRAVITY: f32 = -0.5;

struct PlayState;

impl SimpleState for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //EntityをWorldに追加
        let sprite_sheet_handle = load_sprite_sheet(data.world);
        set_camera(data.world);
        set_rock(data.world,sprite_sheet_handle.clone());
        set_obstacle(data.world,sprite_sheet_handle);
    }

    // PlayStateがPopされるときに実行される
    fn on_stop(&mut self , data: StateData<'_, GameData<'_, '_>>){
        data.world.delete_all();
    }
}

pub fn set_camera(world: &mut World){
    let mut camera_transform = Transform::default(); // カメラの位置を調整するためのComponent
    camera_transform.set_translation_xyz(SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2., 1.0); //スクリーンの中心に設置、高さは1.0のところ
    world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(); 
}

/// 岩EntityをWorldに追加します
pub fn set_rock(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut rock_transform = Transform::default();
    rock_transform.set_translation_xyz(SCREEN_WIDTH / 4., 0., 0.);
    let rock_sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0, //SpriteSheet中の画像の1つ目
    };

    world
        .create_entity()
        .with(rock_transform)
        .with(Rock::new())
        .with(rock_sprite_render)
        .build();
}
/// 岩の情報を保持するComponent
pub struct Rock{
    y: f32,
    velocity: f32,
}

impl Rock{
    pub fn new() -> Rock{
        Rock{
            y: 100. ,
            velocity: 0. ,
        }
    }
}

/// ComponentというTraitを用いてRockをComponentにする
impl Component for Rock {
    type Storage = DenseVecStorage<Self>;
}

/// 障害物EntityをWorldに追加します
pub fn set_obstacle(world: &mut World, sprite_sheet_handle : Handle<SpriteSheet>){

    let mut obstacle_transform = Transform::default();
    obstacle_transform.set_translation_xyz(SCREEN_HEIGHT -10. , OBSTACLE_HEIGHT /2. - 30. ,0.);
    let obstacle_sprite_render = SpriteRender{
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    world.create_entity()
        .with(obstacle_transform)
        .with(Obstacle::new())
        .with(obstacle_sprite_render)
        .build();
}

/// 障害物の情報を保持するComponent
pub struct Obstacle {
    x: f32, // 障害物は右から左にいくだけなのでx座標のみ
}

impl Obstacle {
    pub fn new() -> Obstacle{
        Obstacle{
            x: SCREEN_HEIGHT -10. ,
        }
    }

    pub fn set_x(&mut self, new_x: f32){
        self.x = new_x;
    }
}
/// ComponentというTraitを用いてObstacleをComponentにする
impl Component for Obstacle {
    type Storage = DenseVecStorage<Self>;
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    //スクリーンサイズなどの設定ファイルのPath
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    //ゲームデータ作成　システム　設定を追加
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new(assets_dir, PlayState, game_data)?;
    game.run();

    Ok(())
}
