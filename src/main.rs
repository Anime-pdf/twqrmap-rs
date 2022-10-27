use twmap::*;
use qrcode_generator::QrCodeEcc;
use ndarray::prelude::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    link: String,
    path: String,
}
// Game Tiles
pub const GAME_TILE_EMPTY: u8 = 0;
pub const GAME_TILE_BORDER: u8 = 1;
pub const GAME_TILE_SPAWN: u8 = 192;
// Map Info
pub const MAP_AUTHOR: &str = "Anime-pdf";
pub const MAP_CREDITS: &str = "github.com/Anime-pdf/twqrcode-rs";
// Tiles
pub const TILE_PATH: &str = "mapres/qrcode.png";
pub const TILE_WHITE: u8 = 1;
pub const TILE_BLACK: u8 = 2;
pub const TILE_BG: u8 = 3;

pub fn create_initial_map() -> TwMap {
    let mut map = TwMap::empty(Version::DDNet06);
    map.info.author = MAP_AUTHOR.to_string();
    map.info.credits = MAP_CREDITS.to_string();
    map.images.push(Image::Embedded(EmbeddedImage::from_file(
        TILE_PATH,
    ).unwrap()));
    map
}

fn main() {
    let args = Cli::parse();

    // generate qrcode
    let result: Vec<Vec<bool>> = qrcode_generator::to_matrix_from_str(args.link, QrCodeEcc::Low).unwrap();
    let map_size: (usize, usize) = (result[0].len()+2, result.len()+2);

    // generate map and tile maps with needed size
    let mut map = create_initial_map();
    let mut tiles: Array2<Tile> = Array2::from_shape_simple_fn(map_size, || Tile::new(GAME_TILE_EMPTY, TileFlags::empty()));
    let mut game_tiles: Array2<GameTile> = Array2::from_shape_simple_fn(map_size, || GameTile::new(GAME_TILE_EMPTY, TileFlags::empty()));

    // generate borders and qrcode itself
    for cursor_x in 0..map_size.0 {
        for cursor_y in 0..map_size.1 {
            if cursor_x == 0 || cursor_x == map_size.0-1 || cursor_y == 0 || cursor_y == map_size.1-1 {
                tiles[[cursor_x, cursor_y]] = Tile::new(TILE_BG, TileFlags::empty());
                game_tiles[[cursor_x, cursor_y]] = GameTile::new(GAME_TILE_BORDER, TileFlags::empty());
            }
            else {
                tiles[[cursor_x, cursor_y]] = Tile::new(if result[cursor_x - 1][cursor_y - 1] == true {TILE_BLACK} else {TILE_WHITE}, TileFlags::empty());
            }
        }
    }

    // set spawn
    game_tiles[[1,1]] = GameTile::new(GAME_TILE_SPAWN, TileFlags::empty());

    // declare layers and assign tile maps to them
    let game_layer = GameLayer {tiles: CompressedData::Loaded(game_tiles)};
    let mut tiles_layer = TilesLayer::new(map_size);
    tiles_layer.image = Some(0);
    tiles_layer.tiles = CompressedData::Loaded(tiles);

    // add layers to group
    let mut physics = Group::physics();
    physics.layers.push(Layer::Game(game_layer));
    physics.layers.push(Layer::Tiles(tiles_layer));

    // add group to map
    map.groups.push(physics);

    // save map
    map.save_file(args.path).expect("Error saving map!");

}
