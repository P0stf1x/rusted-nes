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
                        self.pattern_table_bit_plane = value & 0b_0001_0000 != 0;
                        // let pattern_table_bit_plane_8x8 = value & 0b_0000_1000 != 0;
                        self.ppudata_write_down = value & 0b_0000_0100 != 0;
                        self.nametable_address = 0x2000 + ((value & 0b_0000_0011) as usize * 0x400);
                    }
                    MemoryEvent {operation: Read, address: 0x2002, value} => { // PPUSTATUS
                        self.clear_vblank();
                        self.ppu_addr_high_byte = true;
                    }
                    MemoryEvent {operation: Write, address: 0x2005, value} => { // PPUSCROLL
                        if self.ppu_addr_high_byte {
                            self.x_offset = value as usize;
                        } else {
                            self.y_offset = value as usize;
                        };
                        self.ppu_addr_high_byte = !self.ppu_addr_high_byte;
                    },
                    MemoryEvent {operation: Write, address: 0x2006, value} => { // PPUADDR
                        if self.ppu_addr_high_byte {
                            self.ppu_addr &= 0b_0000_0000_1111_1111;
                            self.ppu_addr += ((value & 0b_0011_1111) as usize) << 8;
                        } else {
                            self.ppu_addr &= 0b_0011_1111_0000_0000;
                            self.ppu_addr += value as usize;
                        };
                        self.ppu_addr_high_byte = !self.ppu_addr_high_byte;
                    },
                    MemoryEvent {operation: Read, address: 0x2007, value: _} => { // PPUDATA
                        let vram_data = self.ppu_memory.read(self.ppu_addr, 1) as u8;
                        self.increment_vram_address();
                        self.write_memory(0x2007, vram_data); // since read is offset by 1 cycle it makes our life easier
                        // TODO: for PAL region reads from pixel palette actually return instantly ;-;
                    },
                    MemoryEvent {operation: Write, address: 0x2007, value} => { // PPUDATA
                        self.ppu_memory.write(self.ppu_addr, value);
                        self.increment_vram_address();
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
            self.ppu_addr += 32;
        } else {
            self.ppu_addr += 1;
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
