// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Not doing this will give ugly errors.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::syscall;

/*use agb::{
    display::tiled::{RegularMap, TileFormat, TileSet, TileSetting, TiledMap, VRamManager},
    include_background_gfx, rng,
};*/

pub mod font;

pub mod text;
use text::print_at;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut bitmap = gba.display.video.bitmap3();

    print_at(&mut bitmap, 1, 0, "Hello, world!");
    print_at(&mut bitmap, 4, 0, " !\"#$%&;()*+,-./:;<=>?@{|}~^_`[\\]\n\n0123456789\n\nABCDEFGHIJKLMNOPQRSTUVWXYZ\n\nabcdefghijklmnopqrstuvwxyz");

    loop {
        syscall::halt();
    }
}
