// Mapper3 implements ines mapper 3 (CNROM)
// https://wiki.nesdev.com/w/index.php/INES_Mapper_003

use super::pager::Page;
use super::pager::PageSizeKb;
use super::CartridgeData;
use super::Mapper;
use super::Mirroring;

pub struct Mapper3 {
    data: CartridgeData,
    chr_0: usize,
}

impl Mapper3 {
    pub fn new(data: CartridgeData) -> Self {
        Mapper3 { data, chr_0: 0 }
    }
}

impl Mapper for Mapper3 {
    fn read_prg_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0xBFFF => self
                .data
                .prg_rom
                .read(Page::First(PageSizeKb::Sixteen), address - 0x8000),
            0xC000..=0xFFFF => self
                .data
                .prg_rom
                .read(Page::Last(PageSizeKb::Sixteen), address - 0xC000),
            a => panic!("bad address: {:04X}", a),
        }
    }

    fn write_prg_byte(&mut self, address: u16, value: u8) {
        if let 0x8000..=0xFFFF = address {
            self.chr_0 = value as usize;
        }
    }

    fn read_chr_byte(&self, address: u16) -> u8 {
        self.data
            .chr_rom
            .read(Page::Number(self.chr_0, PageSizeKb::Eight), address)
    }

    fn write_chr_byte(&mut self, _: u16, _: u8) {}

    fn mirroring(&self) -> Mirroring {
        self.data.header.mirroring
    }
}
