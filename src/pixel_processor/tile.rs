use super::PPU_MEM;

#[derive(Clone, Copy)]
enum PixelPalette {
    Background,
    Color1,
    Color2,
    Color3,
}

pub struct Tile {
    data: [PixelPalette; 64],
    color_palette: [u32; 4],
}

impl Tile {
    pub fn get(ppu_memory: &PPU_MEM, tile_id: usize, plane1: bool) -> Self {
        let mut data = [PixelPalette::Background; 64];
        for i in 0..64 {
            let lsb_strip_address = (
                if plane1 {0b_0001_0000_0000_0000} else {0} +
                (tile_id << 4) +
                0b_0000 +      // bit plane offset
                (i / 8)        // strip offset
            );
            let msb_strip_address = (
                if plane1 {0b_0001_0000_0000_0000} else {0} +
                (tile_id << 4) +
                0b_1000 +      // bit plane offset
                (i / 8)        // strip offset
            );
            let bit_offset = 0b_1000_0000 >> i%8;
            let lsb = (ppu_memory.read(lsb_strip_address, 1) & bit_offset) != 0;
            let msb = (ppu_memory.read(msb_strip_address, 1) & bit_offset) != 0;
            data[i] = match (msb, lsb) {
                (false, false) => PixelPalette::Background,
                (false,  true) => PixelPalette::Color1,
                ( true, false) => PixelPalette::Color2,
                ( true,  true) => PixelPalette::Color3,
            };
        };
        return Tile {
            data,
            color_palette: [0xFF000000, 0xFF0000FF, 0xFFFF00FF, 0xFF00FFFF], // TEMP
        };
    }

    pub fn rendered(&self) -> [u32; 64] {
        let mut rendered = [0u32; 64];
        for i in 0..64 {
            rendered[i] = match self.data[i] {
                PixelPalette::Background => self.color_palette[0],
                PixelPalette::Color1 => self.color_palette[1],
                PixelPalette::Color2 => self.color_palette[2],
                PixelPalette::Color3 => self.color_palette[3],
            };
        };
        return rendered;
    }
}
