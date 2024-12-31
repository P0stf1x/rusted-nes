use super::{ines::iNESData, MEM};

pub mod mapper0;

pub fn map(input: iNESData) -> MEM {
    match input.header.mapper_number {
        0 => mapper0::map(input),
        _ => unimplemented!() // TODO: implement other mappers
    }
}

fn check_if_correct_mapper(used_mapper: u16, expected_mapper: u16) {
    if used_mapper != expected_mapper {
        panic!("Wrong mapper used. Used mapper {used_mapper}, expected mapper {expected_mapper}")
    }
}
