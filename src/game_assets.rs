use bevy::{prelude::*, utils::HashMap};

const PLAYER: &str = "sprites/capman.png";
// const ENEMY: &str = "";
const WALLVERTICAL: &str = "sprites/vertical.png";
const WALLHORIZONTAL: &str = "sprites/horizontal.png";
const WALLTOPLEFT: &str = "sprites/top-left.png";
const WALLTOPRIGHT: &str = "sprites/top-right.png";
const WALLBOTTOMLEFT: &str = "sprites/bottom-left.png";
const WALLBOTTOMRIGHT: &str = "sprites/bottom-right.png";
const DOT: &str = "sprites/dot.png";
const POWERPILL: &str = "sprites/powerpill.png";
const UI_FONT: &str = "fonts/pixelplay.ttf";

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssetsLoader>()
            .add_systems(PreStartup, load_assets);
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum GameAssets {
    Player,
    // Enemy,
    WallVertical,
    WallHorizontal,
    WallTopLeft,
    WallTopRight,
    WallBottomLeft,
    WallBottomRight,
    Dot,
    PowerPill,
}

#[derive(Resource)]
pub struct UiFont {
    pub default: Handle<Font>,
}

impl GameAssets {
    fn iterator() -> std::slice::Iter<'static, Self> {
        static GAME_ASSETS: [GameAssets; 9] = [
            GameAssets::Player,
            // GameAssets::Enemy,
            GameAssets::WallVertical,
            GameAssets::WallHorizontal,
            GameAssets::WallTopLeft,
            GameAssets::WallTopRight,
            GameAssets::WallBottomLeft,
            GameAssets::WallBottomRight,
            GameAssets::Dot,
            GameAssets::PowerPill,
        ];
        GAME_ASSETS.iter()
    }

    const fn get_file(&self) -> &str {
        match self {
            Self::Player => PLAYER,
            // Self::Enemy => ENEMY,
            Self::WallVertical => WALLVERTICAL,
            Self::WallHorizontal => WALLHORIZONTAL,
            Self::WallTopLeft => WALLTOPLEFT,
            Self::WallTopRight => WALLTOPRIGHT,
            Self::WallBottomLeft => WALLBOTTOMLEFT,
            Self::WallBottomRight => WALLBOTTOMRIGHT,
            Self::Dot => DOT,
            Self::PowerPill => POWERPILL,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameAssetsLoader {
    assets: HashMap<GameAssets, Handle<Image>>,
}

impl GameAssetsLoader {
    pub fn get(&self, asset: GameAssets) -> Handle<Image> {
        if let Some(handle) = self.assets.get(&asset) {
            return handle.clone();
        }
        panic!("Asset not found or not loaded: {}", asset.get_file());
    }
}

fn load_assets(
    mut commands: Commands,
    mut game_assets: ResMut<GameAssetsLoader>,
    asset_server: Res<AssetServer>,
) {
    for asset in GameAssets::iterator() {
        let handle = asset_server.load(asset.get_file());
        game_assets.assets.insert(*asset, handle);
    }

    let handle: Handle<Font> = asset_server.load(UI_FONT);
    commands.insert_resource(UiFont { default: handle });
}
