use twmap::*;
use qrcode_generator::QrCodeEcc;
use ndarray::prelude::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    link: String,
    path: String,
}

pub const TILE_EMPTY: u8 = 0;
pub const TILE_HOOKABLE: u8 = 1;
pub const TILE_SPAWN: u8 = 192;

pub fn create_initial_map() -> TwMap {
    let mut map = TwMap::empty(Version::DDNet06);
    map.info.author = "Anime.pdf".to_string();
    map.info.credits = "github.com/Anime-pdf/twqrcode-rs".to_string();
    map.images.push(Image::Embedded(EmbeddedImage::from_file(
        "mapres/qrcode.png",
    ).unwrap()));
    map
}

fn main() {
    let args = Cli::parse();
    let mut map = create_initial_map();
    let result: Vec<Vec<bool>> = qrcode_generator::to_matrix_from_str(args.link, QrCodeEcc::Low).unwrap();
    let map_size: (usize, usize) = (result[0].len()+2, result.len()+2);
    let mut tiles: Array2<Tile> = Array2::from_shape_simple_fn(map_size, || Tile::new(TILE_EMPTY, TileFlags::empty()));
    let mut game_tiles: Array2<GameTile> = Array2::from_shape_simple_fn(map_size, || {
        GameTile::new(0, TileFlags::empty())
    });
    for cursor_x in 0..map_size.0 {
        for cursor_y in 0..map_size.1 {
            if cursor_x == 0 || cursor_x == map_size.0-1 || cursor_y == 0 || cursor_y == map_size.1-1 {
                tiles[[cursor_x, cursor_y]] = Tile::new(3, TileFlags::empty());
                game_tiles[[cursor_x, cursor_y]] = GameTile::new(TILE_HOOKABLE, TileFlags::empty());
            }
            else {
                tiles[[cursor_x, cursor_y]] = Tile::new(u8::from(result[cursor_x - 1][cursor_y - 1]) + 1, TileFlags::empty());
            }
        }
    }
    game_tiles[[1,1]] = GameTile::new(TILE_SPAWN, TileFlags::empty());

    let game_layer = GameLayer {
        tiles: CompressedData::Loaded(game_tiles),
    };

    let mut qr = TilesLayer::new(map_size);
    qr.image = Some(0);
    qr.tiles = CompressedData::Loaded(tiles);

    let mut physics = Group::physics();
    physics.layers.push(Layer::Game(game_layer));
    physics.layers.push(Layer::Tiles(qr));

    map.groups.push(physics);

    map.save_file(args.path).expect("TODO: panic message");

}
