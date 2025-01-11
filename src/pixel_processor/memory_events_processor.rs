use super::{ MemoryEvent, MemoryOperation::*, PPU };

impl PPU {
    pub(super) fn process_memory_events(&mut self) {
        match self.memory_events_rx.try_recv() {
            Ok(event) => {
                match event {
                    MemoryEvent {operation: Read, address: 0x2002, value} => { // PPUSTATUS
                        self.clear_vblank();
                        self.w = false;
                    }
                    MemoryEvent {operation: Write, address: 0x2006, value} => { // PPUADDR
                        let value = if !self.w {((value & 0b_0011_1111) as usize) << 8} else {value as usize};
                        self.vram_address &= if self.w {0b_0011_1111_0000_0000} else {0b_0000_0000_1111_1111};
                        self.vram_address += value;
                        self.w = !self.w;
                    },
                    MemoryEvent {operation: Read, address: 0x2007, value: _} => { // PPUDATA
                        let vram_data = self.ppu_memory.read(self.vram_address, 1) as u8;
                        self.increment_vram_address();
                        self.write_memory(0x2007, vram_data); // since read is offset by 1 cycle it makes our life easier
                        // TODO: for PAL region reads from pixel palette actually return instantly ;-;
                    },
                    MemoryEvent {operation: Write, address: 0x2007, value} => { // PPUDATA
                        self.ppu_memory.write(self.vram_address, value);
                        self.increment_vram_address();
                    },
                    _ => (),
                }
            },
            Err(_) => ()
        }
    }

    fn increment_vram_address(&mut self) {
        self.vram_address += 1; // FIXME: if PPUCTRL increment bit enabled it should be 32
    }

    #[allow(unused)]
    fn read_memory(&self, address: usize) -> u8 {
        return unsafe{(*self.memory_pointer.0).data[address]};
    }
    fn write_memory(&self, address: usize, value: u8) {
        unsafe{(*self.memory_pointer.0).data[address] = value};
    }
}
