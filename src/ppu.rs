extern crate sdl2;
use std::cell::RefCell;

use sdl2::{
    pixels::{Color, PixelFormatEnum},
    render::Texture,
    Sdl,
};

use crate::gameboy;

pub struct PPU {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    texture: RefCell<sdl2::render::Texture<'static>>,
    frambuffer_alpha: [u32; 160 * 144],
}

macro_rules! flag_set_at {
    ($byte:expr, $bit:expr) => {
        ($byte >> $bit) & 1 == 1
    };
}

const WHITE: Color = Color::WHITE;
const LIGHT_GRAY: Color = Color::RGBA(0xCC, 0xCC, 0xCC, 0xFF);
const DARK_GRAY: Color = Color::RGBA(0x77, 0x77, 0x77, 0xFF);
const BLACK: Color = Color::BLACK;

impl PPU {
    pub fn new(sdl_context: &Sdl) -> PPU {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let texture_creator = canvas.texture_creator();

        let texture = texture_creator
            .create_texture(
                PixelFormatEnum::ABGR32,
                sdl2::render::TextureAccess::Streaming,
                160,
                144,
            )
            .unwrap();

        let texture = unsafe { std::mem::transmute::<_, Texture<'static>>(texture) };
        PPU {
            canvas,
            texture_creator,
            texture: RefCell::new(texture),
            frambuffer_alpha: [0; 160 * 144],
        }
    }

    pub fn update(&mut self, gameboy: &gameboy::Gameboy, current_scanline: u8) {
        // self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        let control = gameboy.bus.read_byte(0xff40);

        if flag_set_at!(control, 0) {
            self.render_tiles(gameboy, current_scanline);
        }

        if flag_set_at!(control, 1) {
            self.render_sprites(gameboy, current_scanline);
        }
    }

    pub fn draw(&mut self) {
        let mut texture = self.texture.borrow_mut();
        texture
            .update(None, self.data_raw(), 160 * 4)
            .expect("Failed to update texture");
        // texture.update(None, pixel_data, pitch);
        self.canvas
            .copy(&texture, None, None)
            .expect("Failed to copy texture");
        self.canvas.present();
    }

    fn render_tiles(&mut self, gameboy: &gameboy::Gameboy, current_scanline: u8) {
        let scroll_y = gameboy.bus.read_byte(0xff42);
        let scroll_x = gameboy.bus.read_byte(0xff43);
        let window_y = gameboy.bus.read_byte(0xff4a);
        let window_x = gameboy.bus.read_byte(0xff4b).wrapping_sub(7);

        let control = gameboy.bus.read_byte(0xff40);

        let using_window = flag_set_at!(control, 5) && window_y <= current_scanline;

        let (tiledata, unsigned) = if flag_set_at!(control, 4) {
            (0x8000, true)
        } else {
            (0x8800, false)
        };

        let tilemap = if !using_window {
            if flag_set_at!(control, 3) {
                0x9c00
            } else {
                0x9800
            }
        } else {
            if flag_set_at!(control, 6) {
                0x9c00
            } else {
                0x9800
            }
        };

        let y_pos = if using_window {
            current_scanline - window_y
        } else {
            scroll_y.wrapping_add(current_scanline)
        };

        let tile_row = (y_pos as u16 / 8) * 32;

        for pixel in 0..160 {
            let mut x_pos = pixel + scroll_x;
            if using_window && pixel >= window_x {
                x_pos = pixel - window_x;
            }

            let tile_col = x_pos / 8;
            let tile_address = tilemap + tile_row + tile_col as u16;

            let tile_num = gameboy.bus.read_byte(tile_address as u16);

            let tile_location = tiledata
                + (if unsigned {
                    tile_num as u16
                } else {
                    ((tile_num as i8 as i16) + 128) as u16
                } * 16);

            let line = ((y_pos % 8) * 2) as u16;
            let data1 = gameboy.bus.read_byte(tile_location + line);
            let data2 = gameboy.bus.read_byte(tile_location + line + 1);

            let color_bit = 7 - (x_pos % 8);

            let mut color_num;

            color_num = ((data2 >> color_bit) & 1) << 1;
            color_num |= (data1 >> color_bit) & 1;

            let color = self.get_color(color_num, gameboy.bus.read_byte(0xFF47));

            if (current_scanline > 143) || (pixel > 159) {
                continue;
            }

            self.frambuffer_alpha[(current_scanline as usize * 160 + (pixel as usize)) as usize] =
                u32::from_be_bytes(color.rgba().into());
        }
    }

    fn data_raw(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.frambuffer_alpha.as_ptr() as *const u8,
                self.frambuffer_alpha.len() * 4,
            )
        }
    }

    fn get_color(&self, color_num: u8, pallete: u8) -> Color {
        let (hi, lo) = match color_num {
            0 => (1, 0),
            1 => (3, 2),
            2 => (5, 4),
            3 => (7, 6),
            _ => panic!("Invalid color number"),
        };

        let color = ((pallete >> hi) & 1) << 1 | ((pallete >> lo) & 1);

        match color {
            0 => WHITE,
            1 => LIGHT_GRAY,
            2 => DARK_GRAY,
            3 => BLACK,
            _ => panic!("Invalid color"),
        }
    }

    fn render_sprites(&mut self, gameboy: &gameboy::Gameboy, current_scanline: u8) {
        let control = gameboy.bus.read_byte(0xff40);
        let use8x16 = flag_set_at!(control, 2);

        for sprite in 0..40 {
            let index = sprite * 4;
            let y_pos = gameboy.bus.read_byte(0xfe00 + index) - 16;
            let x_pos = gameboy.bus.read_byte(0xfe00 + index + 1) - 8;
            let tile_location = gameboy.bus.read_byte(0xfe00 + index + 2);
            let attributes = gameboy.bus.read_byte(0xfe00 + index + 3);

            let x_flip = flag_set_at!(attributes, 5);
            let y_flip = flag_set_at!(attributes, 6);

            let y_size = if use8x16 { 16 } else { 8 };

            // Check if sprite is on this line
            if (current_scanline >= y_pos) && (current_scanline < y_pos + y_size) {
                let mut line = (current_scanline - y_pos) as i8;

                if y_flip {
                    line -= y_size as i8;
                    line *= -1;
                }

                line *= 2;

                let data_address = 0x8000 + (tile_location as u16 * 16) + (line as u16);

                let data1 = gameboy.bus.read_byte(data_address);
                let data2 = gameboy.bus.read_byte(data_address + 1);

                for tile_pixel in 7..=0 {
                    let mut color_bit = tile_pixel as i8;
                    if x_flip {
                        color_bit -= 7;
                        color_bit *= -1;
                    }

                    let mut color_num = ((data2 >> color_bit) & 1) << 1;
                    color_num |= (data1 >> color_bit) & 1;

                    let color = self.get_color(color_num, gameboy.bus.read_byte(0xFF48));
                    if color == WHITE {
                        continue;
                    }

                    let pixel = x_pos + 7 - tile_pixel;

                    if (current_scanline > 143) || (pixel > 159) {
                        continue;
                    }

                    self.frambuffer_alpha
                        [(current_scanline as usize * 160 + (pixel as usize)) as usize] =
                        u32::from_be_bytes(color.rgba().into());
                }
            }
        }
    }
}
