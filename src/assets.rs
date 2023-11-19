use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource, Default)]
pub struct MyAssets
{
    #[asset(path = "man_transp.png")]
    pub player: Handle<Image>,
    #[asset(path = "rock.png")]
    pub rock: Handle<Image>,
    #[asset(path = "water.png")]
    pub water: Handle<Image>,
    #[asset(path = "grass_var1.png")]
    pub grass: Handle<Image>,
    #[asset(path = "feesh_man_sheet.png")]
    pub merchant: Handle<Image>,
    #[asset(path = "cart.png")]
    pub cart: Handle<Image>,
    #[asset(path = "fish_man.png")]
    pub fish_man: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 8, rows = 1))]
    #[asset(path = "player_sheet.png")]
    pub player_moving: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 2, rows = 1))]
    #[asset(path = "slash_sheet.png")]
    pub slash: Handle<TextureAtlas>,
}
