use super::{ MemoryEvent, MemoryOperation::*, PPU };

impl PPU {
    pub(super) fn process_memory_events(&mut self) {
        match self.memory_events_rx.try_recv() {
            Ok(event) => {
                match event {
                    MemoryEvent {operation: Write, address: 0x2000, value} => { // PPUCTRL
                        self.nmi_enabled = value & 0b_1000_0000 != 0;
                        // let ppu_master = value & 0b_0100_0000 != 0;
                        // let two_high_sprites = value & 0b_0010_0000 != 0;
                        self.bg_plane = value & 0b_0001_0000 != 0;
                        self.fg_plane = value & 0b_0000_1000 != 0;
                        self.ppudata_write_down = value & 0b_0000_0100 != 0;
                        self.vram_t.set_nametable_v((value & 0b_0000_0010) != 0);
                        self.vram_t.set_nametable_h((value & 0b_0000_0001) != 0);
                    }
                    MemoryEvent {operation: Write, address: 0x2001, value} => { // PPUMASK
                        // self.emphasize_b = value & 0b_1000_0000 != 0;
                        // self.emphasize_g = value & 0b_0100_0000 != 0;
                        // self.emphasize_r = value & 0b_0010_0000 != 0;
                        self.fg_rendering = value & 0b_0001_0000 != 0;
                        self.bg_rendering = value & 0b_0000_1000 != 0;
                        // self.fg_left_rendering = value & 0b_0000_0100 != 0;
                        // self.bg_left_rendering = value & 0b_0000_0010 != 0;
                        // self.greyscale_rendering = value & 0b_0000_0001 != 0;
                    }
                    MemoryEvent {operation: Read, address: 0x2002, value} => { // PPUSTATUS
                        self.clear_vblank();
                        self.ppu_addr_high_byte = true;
                    }
                    MemoryEvent {operation: Write, address: 0x2003, value} => { // OAMADDR
                        self.oam_addr = value as usize;
                        unsafe {
                            (&mut *self.memory_pointer.0).data[0x2004] = self.oam_data[self.oam_addr];
                        }
                    },
                    MemoryEvent {operation: Read, address: 0x2004, value} => { // OAMDATA
                        // actually there's nothing to do, since we write expected data when changing addr
                    },
                    MemoryEvent {operation: Write, address: 0x2004, value} => { // OAMDATA
                        self.oam_data[self.oam_addr] = value;
                        self.oam_addr = (self.oam_addr + 1) & 0xFF;
                        unsafe {
                            // (&mut *self.memory_pointer.0).write_no_hook(0x2004, self.oam_data[self.oam_addr]);
                            (&mut *self.memory_pointer.0).data[0x2004] = self.oam_data[self.oam_addr];
                        }
                    },
                    MemoryEvent {operation: Write, address: 0x2005, value} => { // PPUSCROLL
                        if self.ppu_addr_high_byte {
                            self.vram_t.set_coarse_x((value & 0b_1111_1000) >> 3);
                            self.fine_x = value & 0b_0000_0111;
                        } else {
                            self.vram_t.set_coarse_y((value & 0b_1111_1000) >> 3);
                            self.vram_t.set_fine_y(value & 0b_0000_0111);
                        };
                        self.ppu_addr_high_byte = !self.ppu_addr_high_byte;
                    },
                    MemoryEvent {operation: Write, address: 0x2006, value} => { // PPUADDR
                        if self.ppu_addr_high_byte {
                            let mut new_vram_t = self.vram_t.get_all();
                            new_vram_t &= 0b_0000_0000_1111_1111;
                            new_vram_t += ((value & 0b_0011_1111) as u16) << 8;
                            self.vram_t.set_all(new_vram_t);
                        } else {
                            let mut new_vram_t = self.vram_t.get_all();
                            new_vram_t &= 0b_0111_1111_0000_0000;
                            new_vram_t += (value as u16);
                            self.vram_t.set_all(new_vram_t);
                            self.vram_v = self.vram_t;
                        };
                        self.ppu_addr_high_byte = !self.ppu_addr_high_byte;
                    },
                    MemoryEvent {operation: Read, address: 0x2007, value: _} => { // PPUDATA
                        let vram_data = self.ppu_memory.read((self.vram_v.get_all() & 0x3FFF) as usize, 1) as u8;
                        self.increment_vram_address();
                        self.write_memory(0x2007, vram_data); // since read is offset by 1 cycle it makes our life easier
                        // TODO: for PAL region reads from pixel palette actually return instantly ;-;
                    },
                    MemoryEvent {operation: Write, address: 0x2007, value} => { // PPUDATA
                        self.ppu_memory.write((self.vram_v.get_all() & 0x3FFF) as usize, value);
                        self.increment_vram_address();
                    },

                    MemoryEvent {operation: Write, address: 0x4014, value} => { // OAMDMA
                        let page = (value as usize) << 8;
                        for address_offset in 0..=0xFF {
                            self.oam_data[address_offset] = unsafe {
                                (&mut *self.memory_pointer.0).read_no_hook(page+address_offset, 1) as u8
                            }
                        };
                    },

                    MemoryEvent {operation: Write, address: 0x4016, value} => { // Controller capture state
                        self.controller_state = self.get_controller_state(); // FIXME: You're actually supposed to read into the shift register only when bit 0 is set, and stop reading when bit 0 is cleared.
                        unsafe {
                            (&mut *self.memory_pointer.0).data[0x4016] = self.controller_state & 0b_0000_0001;
                        }
                    },
                    MemoryEvent {operation: Read, address: 0x4016, value} => { // Controller 1 read
                        self.controller_state = self.controller_state >> 1;
                        self.controller_state |= 0b_1000_0000;
                        unsafe {
                            (&mut *self.memory_pointer.0).data[0x4016] = self.controller_state & 0b_0000_0001;
                        }
                    },
                    _ => (),
                }
            },
            Err(_) => ()
        }
    }

    fn increment_vram_address(&mut self) {
        if self.ppudata_write_down {
            self.vram_v.set_all(self.vram_v.get_all() + 32);
        } else {
            self.vram_v.set_all(self.vram_v.get_all() + 1);
        }
    }

    #[allow(unused)]
    fn read_memory(&self, address: usize) -> u8 {
        return unsafe{(&mut *self.memory_pointer.0).data[address]};
    }
    fn write_memory(&self, address: usize, value: u8) {
        unsafe{(&mut *self.memory_pointer.0).data[address] = value};
    }
}
