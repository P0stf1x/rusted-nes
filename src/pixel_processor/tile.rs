use crate::pixel_processor::helper::reverse_bits;

use super::PPU_MEM;

#[derive(Clone, Copy)]
pub enum PixelPaletteColorIndex {
    Background,
    Color1,
    Color2,
    Color3,
}

#[derive(Clone, Copy)]
pub struct PixelPalette {
    pub background: u32,
    pub color1: u32,
    pub color2: u32,
    pub color3: u32,
}

impl PixelPalette {
    pub fn get_by_id(ppu_memory: &PPU_MEM, palette_id: usize) -> Self {
        let palette_addr = ppu_memory.read(0x3F00 + palette_id*4, 4);
        return Self {
            background: get_color(palette_addr as u8),
            color1: get_color((palette_addr>>8) as u8),
            color2: get_color((palette_addr>>16) as u8),
            color3: get_color((palette_addr>>24) as u8),
        };
    }

    pub fn get_from_addr_and_offset(ppu_memory: &PPU_MEM, nametable_address: usize, tile_offset: usize) -> Self {
        let tile_attribute_x = (tile_offset & 0b_0000_0010) >> 1;
        let tile_attribute_y = (tile_offset & 0b_0100_0000) >> 6;

        let attribute_byte_x_part = (tile_offset & 0b_0000_0001_1100) >> 2;
        let attribute_byte_y_part = (tile_offset & 0b_0011_1000_0000) >> 4;
        let attribute_byte_offset = attribute_byte_x_part + attribute_byte_y_part;

        let attribute_byte = ppu_memory.read(nametable_address + 0x3C0 + attribute_byte_offset, 1);

        let palette_index = match (tile_attribute_x, tile_attribute_y) {
            (0, 0) => attribute_byte & 0b_0000_0011,
            (1, 0) => (attribute_byte & 0b_0000_1100) >> 2,
            (0, 1) => (attribute_byte & 0b_0011_0000) >> 4,
            (1, 1) => (attribute_byte & 0b_1100_0000) >> 6,
            _ => panic!()
        };
        let palette = ppu_memory.read(0x3F00 + palette_index*4, 4);
        return Self {
            background: get_color(ppu_memory.read(0x3F00, 1) as u8),
            color1: get_color((palette>>8) as u8),
            color2: get_color((palette>>16) as u8),
            color3: get_color((palette>>24) as u8),
        };
    }

    pub fn get_sample_palette() -> Self {
        Self {
            background: 0xFF000000,
            color1: 0xFF0000FF,
            color2: 0xFFFF00FF,
            color3: 0xFF00FFFF,
        }
    }
}

fn get_color(index: u8) -> u32 {
    if index >= 64 {panic!()}
    let color = [ // TODO: allow to change it/set it with .pal files. Currently it is recommended palette from https://www.nesdev.org/wiki/PPU_palettes
        0xFF626262, 0xFF002E98, 0xFF0C11C2, 0xFF3B00C2, 0xFF650098, 0xFF7D004E, 0xFF7D0000, 0xFF651900, 0xFF3B3600, 0xFF0C4F00, 0xFF005B00, 0xFF005900, 0xFF00494E, 0xFF000000, 0xFF000000, 0xFF000000,
        0xFFABABAB, 0xFF0064F4, 0xFF353CFF, 0xFF761BFF, 0xFFAE0AF4, 0xFFCF0C8F, 0xFFCF231C, 0xFFAE4700, 0xFF766F00, 0xFF359000, 0xFF00A100, 0xFF009E1C, 0xFF00888F, 0xFF000000, 0xFF000000, 0xFF000000,
        0xFFFFFFFF, 0xFF4AB5FF, 0xFF858CFF, 0xFFC86AFF, 0xFFFF58FF, 0xFFFF5BE2, 0xFFFF726A, 0xFFFF9702, 0xFFC8C100, 0xFF85E300, 0xFF4AF502, 0xFF29F26A, 0xFF29DBE2, 0xFF4E4E4E, 0xFF000000, 0xFF000000,
        0xFFFFFFFF, 0xFFB6E1FF, 0xFFCED1FF, 0xFFE9C3FF, 0xFFFFBCFF, 0xFFFFBDF4, 0xFFFFC6C3, 0xFFFFD59A, 0xFFE9E681, 0xFFCEF481, 0xFFB6FB9A, 0xFFA9FAC3, 0xFFA9F0F4, 0xFFB8B8B8, 0xFF000000, 0xFF000000
    ];
    return color[index as usize];
}

pub struct Tile {
    data: [PixelPaletteColorIndex; 64]
}

impl Tile {
    pub fn get(ppu_memory: &PPU_MEM, tile_pattern_id: usize, plane1: bool, reverse_h: bool, reverse_v: bool) -> Self {
        let mut data = [PixelPaletteColorIndex::Background; 64];
        for y in 0..8 {
            let reverse_aware_y = if reverse_v {
                7-y
            } else {
                y
            };
            for x in 0..8 {
                let lsb_strip_address = (
                    if plane1 {0b_0001_0000_0000_0000} else {0} +
                    (tile_pattern_id << 4) +
                    0b_0000 +       // bit plane offset
                    reverse_aware_y // strip offset
                );
                let msb_strip_address = (
                    if plane1 {0b_0001_0000_0000_0000} else {0} +
                    (tile_pattern_id << 4) +
                    0b_1000 +       // bit plane offset
                    reverse_aware_y // strip offset
                );
                let bit_offset = 0b_1000_0000 >> x;
                let mut lsb_value = ppu_memory.read(lsb_strip_address, 1) as u8;
                let mut msb_value = ppu_memory.read(msb_strip_address, 1) as u8;
                if reverse_h {
                    lsb_value = reverse_bits(lsb_value);
                    msb_value = reverse_bits(msb_value);
                }
                let lsb = (lsb_value & bit_offset) != 0;
                let msb = (msb_value & bit_offset) != 0;
                data[x + y*8] = match (msb, lsb) {
                    (false, false) => PixelPaletteColorIndex::Background,
                    (false,  true) => PixelPaletteColorIndex::Color1,
                    ( true, false) => PixelPaletteColorIndex::Color2,
                    ( true,  true) => PixelPaletteColorIndex::Color3,
                };
            };
        };
        return Tile {
            data
        };
    }

    pub fn get_at(ppu_memory: &PPU_MEM, x: usize, y: usize, tile_pattern_id: usize, plane1: bool) -> PixelPaletteColorIndex {
        let lsb_strip_address = (
            if plane1 {0b_0001_0000_0000_0000} else {0} +
            (tile_pattern_id << 4) +
            0b_0000 +      // bit plane offset
            y              // strip offset
        );
        let msb_strip_address = (
            if plane1 {0b_0001_0000_0000_0000} else {0} +
            (tile_pattern_id << 4) +
            0b_1000 +      // bit plane offset
            y              // strip offset
        );
        let bit_offset = 0b_1000_0000 >> x;
        let lsb = (ppu_memory.read(lsb_strip_address, 1) & bit_offset) != 0;
        let msb = (ppu_memory.read(msb_strip_address, 1) & bit_offset) != 0;
        match (msb, lsb) {
            (false, false) => PixelPaletteColorIndex::Background,
            (false,  true) => PixelPaletteColorIndex::Color1,
            ( true, false) => PixelPaletteColorIndex::Color2,
            ( true,  true) => PixelPaletteColorIndex::Color3,
        }
    }

    pub fn rendered(&self, palette: PixelPalette) -> [u32; 64] {
        let mut rendered = [0u32; 64];
        for i in 0..64 {
            rendered[i] = match self.data[i] {
                PixelPaletteColorIndex::Background => palette.background,
                PixelPaletteColorIndex::Color1 => palette.color1,
                PixelPaletteColorIndex::Color2 => palette.color2,
                PixelPaletteColorIndex::Color3 => palette.color3,
            };
        };
        return rendered;
    }
}
