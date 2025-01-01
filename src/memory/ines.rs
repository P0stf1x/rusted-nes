// use nom;
use nom::bytes::complete::{ tag, take };
use nom::IResult;

#[allow(non_camel_case_types)]
pub struct iNESData {
    pub header: iNESHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct iNESHeader {
    pub version: iNESVersion,
    pub prg_rom_size: usize,
    pub chr_rom_size: usize,
    pub mapper_number: u16,
    pub trainer_enabled: bool,
    pub battery_enabled: bool,
    pub alternative_nametables: bool,
    pub nametable_layout: NametableLayout,
    pub console_type: ConsoleType,
    pub prg_ram_size: usize,
    // iNES 2.0 specific
    pub submapper_number: Option<u8>,
    pub prg_nvram_size: Option<usize>,
    pub chr_ram_size: Option<usize>,
    pub chr_nvram_size: Option<usize>,
    pub console_timing: Option<ConsoleTiming>,
    // TODO: there's still more, but those are pretty much the basics
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum iNESVersion {
    iNES_1,
    iNES_2,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum NametableLayout {
    vertical_or_mapper,
    horizontal,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ConsoleType {
    NES_FamilyComputer,
    VS_System,
    Playchoice_10,
    ExtendedConsoleType,
}

#[derive(Debug)]
pub enum ConsoleTiming {
    NTSC,
    PAL,
    MultiRegion,
    Dendy,
}

pub fn parse_file(input: &[u8]) -> iNESData {
    let (remaining, header) = match get_header(input) {
        Err(_) => panic!("Couldn't read header (input is empty or < 16 bytes?)"),
        Ok((remaining, header_data)) => (remaining, parse_header(header_data))
    };
    if header.trainer_enabled {unimplemented!("No trainer support")}; // TODO: implemet
    let (remaining, prg_rom_data) = match get_prg_rom_data(remaining, header.prg_rom_size) {
        Err(_) => panic!("Can't read PRG ROM data section"),
        Ok((remaining, prg_rom_data)) => (remaining, prg_rom_data)
    };
    let (remaining, chr_rom_data) = match get_chr_rom_data(remaining, header.chr_rom_size) {
        Err(_) => panic!("Can't read CHR ROM data section"),
        Ok((remaining, chr_rom_data)) => (remaining, chr_rom_data)
    };
    if !remaining.is_empty() {unimplemented!("Playchoice 10 not implemented")}; // TODO: implemet
    return iNESData {
        header,
        prg_rom: prg_rom_data.to_vec(),
        chr_rom: chr_rom_data.to_vec(),
    };
}

pub fn get_prg_rom_data(input: &[u8], chunked_length: usize) -> IResult<&[u8], &[u8]> {
    take(chunked_length*1024*16)(input)
}

pub fn get_chr_rom_data(input: &[u8], chunked_length: usize) -> IResult<&[u8], &[u8]> {
    take(chunked_length*1024*8)(input)
}

fn parse_header(header: &[u8]) -> iNESHeader {
    let remaining_header = match parse_magic(header) {
        Err(_) => panic!("Invalid iNES magic value"),
        Ok((remaining_header, _)) => remaining_header
    };
    let (remaining_header, prg_rom_size_lsb, chr_rom_size_lsb) = match get_prg_chr_roms_size(remaining_header) {
        Err(_) => panic!("Can't read header bytes 4-5"),
        Ok((remaining_header, (prg_rom_size_lsb, chr_rom_size_lsb))) => (remaining_header, prg_rom_size_lsb, chr_rom_size_lsb)
    };
    let (remaining_header, mapper_b0_b3) = match parse_multiple_bits((remaining_header, 0), 4) {
        Err(_) => panic!("Can't read mapper bits 0-3"),
        Ok((remaining_header, mapper_b0_b3)) => (remaining_header, mapper_b0_b3)
    };
    let (remaining_header, alternative_nametables) = match parse_bit(remaining_header) {
        Err(_) => panic!("Can't read alternative nametables bit"),
        Ok((remaining_header, alternative_nametables)) => (remaining_header, alternative_nametables)
    };
    let (remaining_header, trainer_enabled) = match parse_bit(remaining_header) {
        Err(_) => panic!("Can't read trainer enabled bit"),
        Ok((remaining_header, trainer_enabled)) => (remaining_header, trainer_enabled)
    };
    let (remaining_header, battery_enabled) = match parse_bit(remaining_header) {
        Err(_) => panic!("Can't read battery enabled bit"),
        Ok((remaining_header, battery_enabled)) => (remaining_header, battery_enabled)
    };
    let (remaining_header, nametable_layout) = match parse_bit(remaining_header) {
        Err(_) => panic!("Can't read nametable layout bit"),
        Ok((remaining_header, true)) => (remaining_header, NametableLayout::horizontal),
        Ok((remaining_header, false)) => (remaining_header, NametableLayout::vertical_or_mapper)
    };
    let (remaining_header, mapper_b4_b7) = match parse_multiple_bits(remaining_header, 4) {
        Err(_) => panic!("Can't read mapper bits 4-7"),
        Ok((remaining_header, mapper_b4_b7)) => (remaining_header, mapper_b4_b7)
    };
    #[allow(non_snake_case)]
    let (remaining_header, iNES_version) = match parse_multiple_bits(remaining_header, 2) {
        Err(_) => panic!("Can't read iNES version"),
        Ok((remaining_header, 0b10)) => (remaining_header, iNESVersion::iNES_2),
        Ok((remaining_header, _)) => (remaining_header, iNESVersion::iNES_1)
    };
    let (remaining_header, console_type) = match parse_multiple_bits(remaining_header, 2) {
        Err(_) => panic!("Can't read console type"),
        Ok((remaining_header, 0b00)) => (remaining_header, ConsoleType::NES_FamilyComputer),
        Ok((remaining_header, 0b01)) => (remaining_header, ConsoleType::VS_System),
        Ok((remaining_header, 0b10)) => (remaining_header, ConsoleType::Playchoice_10),
        Ok((remaining_header, 0b11)) => (remaining_header, ConsoleType::ExtendedConsoleType),
        _ => panic!("Somehow there's more than 2 bits of information in 2 bits of console type?")
    };
    match iNES_version {
        iNESVersion::iNES_1 => {
            // TV system unimplemented // flag 9 //
            // TV system, PRG-RAM presence unimplemented // flag 10 //
            // bytes 11-15 unused
            let (_remaining_header, prg_ram_size) = match take::<usize, &[u8], nom::error::Error<_>>(1usize)(remaining_header.0) {
                Err(_) => panic!("Can't read PRG RAM size"),
                Ok((remaining_header, prg_ram_size)) => (remaining_header, prg_ram_size[0])
            };
            return iNESHeader {
                version: iNES_version,
                prg_rom_size: prg_rom_size_lsb as usize,
                chr_rom_size: chr_rom_size_lsb as usize,
                mapper_number: ((mapper_b4_b7 as u16)<<4)+mapper_b0_b3 as u16,
                trainer_enabled,
                battery_enabled,
                alternative_nametables,
                nametable_layout,
                console_type,
                prg_ram_size: prg_ram_size as usize,
                // iNES 2.0 stuff
                submapper_number: None,
                prg_nvram_size: None,
                chr_ram_size: None,
                chr_nvram_size: None,
                console_timing: None,
            };
        }
        iNESVersion::iNES_2 => {
            let (remaining_header, submapper_number) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read submapper number"),
                Ok((remaining_header, submapper_number)) => (remaining_header, submapper_number)
            };
            let (remaining_header, mapper_b8_b11) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read mapper bits 8-11"),
                Ok((remaining_header, mapper_b8_b11)) => (remaining_header, mapper_b8_b11)
            };
            let (remaining_header, prg_rom_size_msb) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read PRG ROM size MSB"),
                Ok((_remaining_header, 0b1111)) => unimplemented!("No exponent PRG ROM support"),
                Ok((remaining_header, prg_rom_size_msb)) => (remaining_header, prg_rom_size_msb)
            };
            let (remaining_header, chr_rom_size_msb) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read CHR ROM size MSB"),
                Ok((_remaining_header, 0b1111)) => unimplemented!("No exponent CHR ROM support"),
                Ok((remaining_header, chr_rom_size_msb)) => (remaining_header, chr_rom_size_msb)
            };
            let (remaining_header, prg_nvram) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read PRG NVRAM"),
                Ok((remaining_header, prg_nvram)) => (remaining_header, prg_nvram)
            };
            let (remaining_header, prg_ram) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read PRG RAM"),
                Ok((remaining_header, prg_ram)) => (remaining_header, prg_ram)
            };
            let (remaining_header, chr_nvram) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read CHR NVRAM"),
                Ok((remaining_header, chr_nvram)) => (remaining_header, chr_nvram)
            };
            let (remaining_header, chr_ram) = match parse_multiple_bits(remaining_header, 4) {
                Err(_) => panic!("Can't read CHR RAM"),
                Ok((remaining_header, chr_ram)) => (remaining_header, chr_ram)
            };
            let remaining_header = match parse_multiple_bits(remaining_header, 6) {
                Err(_) => panic!("Header too short?"),
                Ok((remaining_header, _)) => remaining_header // byte 12 bits 2-7 are unused
            };
            let (_remaining_header, console_timing) = match parse_multiple_bits(remaining_header, 2) {
                Err(_) => panic!("Can't read console timing"),
                Ok((remaining_header, 0b00)) => (remaining_header, ConsoleTiming::NTSC),
                Ok((remaining_header, 0b01)) => (remaining_header, ConsoleTiming::PAL),
                Ok((remaining_header, 0b10)) => (remaining_header, ConsoleTiming::MultiRegion),
                Ok((remaining_header, 0b11)) => (remaining_header, ConsoleTiming::Dendy),
                _ => panic!("Somehow there's more than 2 bits of information in 2 bits of console timing?")
            };
            // TODO: bytes 13-15 are too obscure for now
            return iNESHeader {
                version: iNES_version,
                prg_rom_size: ((prg_rom_size_msb as usize)<<8) + prg_rom_size_lsb as usize,
                chr_rom_size: ((chr_rom_size_msb as usize)<<8) + chr_rom_size_lsb as usize,
                mapper_number: ((mapper_b8_b11 as u16)<<8)+((mapper_b4_b7 as u16)<<4)+mapper_b0_b3 as u16,
                trainer_enabled,
                battery_enabled,
                alternative_nametables,
                nametable_layout,
                console_type,
                submapper_number: Some(submapper_number),
                prg_nvram_size: Some(64<<prg_nvram),
                prg_ram_size: 64<<prg_ram,
                chr_nvram_size: Some(64<<chr_nvram),
                chr_ram_size: Some(64<<chr_ram),
                console_timing: Some(console_timing),
            };
        }
    }
}

// Input is a tuple of (input: I, bit_offset: usize)
fn parse_multiple_bits(input: (&[u8], usize), count: usize)-> IResult<(&[u8], usize), u8> {
        nom::bits::complete::take(count)(input)
    }

fn parse_bit(input: (&[u8], usize))-> IResult<(&[u8], usize), bool> {
        match parse_multiple_bits(input, 1) {
            Err(e) => Err(e),
            Ok((remaining, bit)) => Ok((remaining, if bit!=0 { true } else { false } ))
        }
    }

fn get_prg_chr_roms_size(input: &[u8]) -> IResult<&[u8], (u8, u8)> {
    match take::<usize, &[u8], nom::error::Error<_>>(2usize)(input) {
        Err(e) => Err(e),
        Ok((remaining_header, rom_size)) => Ok((remaining_header, (rom_size[0], rom_size[1])))
    }
}
fn get_header(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(16usize)(input)
}

fn parse_magic(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(b"NES\x1A")(input)
}
