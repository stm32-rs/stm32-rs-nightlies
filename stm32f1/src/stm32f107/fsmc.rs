#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcr1: BCR1,
    btr1: BTR1,
    bcr2: BCR2,
    btr2: BTR2,
    bcr3: BCR3,
    btr3: BTR3,
    bcr4: BCR4,
    btr4: BTR4,
    _reserved8: [u8; 0x40],
    pcr2: PCR2,
    sr2: SR2,
    pmem2: PMEM2,
    patt2: PATT2,
    _reserved12: [u8; 0x04],
    eccr2: ECCR2,
    _reserved13: [u8; 0x08],
    pcr3: PCR3,
    sr3: SR3,
    pmem3: PMEM3,
    patt3: PATT3,
    _reserved17: [u8; 0x04],
    eccr3: ECCR3,
    _reserved18: [u8; 0x08],
    pcr4: PCR4,
    sr4: SR4,
    pmem4: PMEM4,
    patt4: PATT4,
    pio4: PIO4,
    _reserved23: [u8; 0x50],
    bwtr1: BWTR1,
    _reserved24: [u8; 0x04],
    bwtr2: BWTR2,
    _reserved25: [u8; 0x04],
    bwtr3: BWTR3,
    _reserved26: [u8; 0x04],
    bwtr4: BWTR4,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR1 {
        &self.btr1
    }
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR2 {
        &self.btr2
    }
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR3 {
        &self.bcr3
    }
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR3 {
        &self.btr3
    }
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR4 {
        &self.bcr4
    }
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR4 {
        &self.btr4
    }
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    #[inline(always)]
    pub const fn pcr2(&self) -> &PCR2 {
        &self.pcr2
    }
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x68 - Common memory space timing register 2"]
    #[inline(always)]
    pub const fn pmem2(&self) -> &PMEM2 {
        &self.pmem2
    }
    #[doc = "0x6c - Attribute memory space timing register 2"]
    #[inline(always)]
    pub const fn patt2(&self) -> &PATT2 {
        &self.patt2
    }
    #[doc = "0x74 - ECC result register 2"]
    #[inline(always)]
    pub const fn eccr2(&self) -> &ECCR2 {
        &self.eccr2
    }
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    #[inline(always)]
    pub const fn pcr3(&self) -> &PCR3 {
        &self.pcr3
    }
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    #[inline(always)]
    pub const fn sr3(&self) -> &SR3 {
        &self.sr3
    }
    #[doc = "0x88 - Common memory space timing register 3"]
    #[inline(always)]
    pub const fn pmem3(&self) -> &PMEM3 {
        &self.pmem3
    }
    #[doc = "0x8c - Attribute memory space timing register 3"]
    #[inline(always)]
    pub const fn patt3(&self) -> &PATT3 {
        &self.patt3
    }
    #[doc = "0x94 - ECC result register 3"]
    #[inline(always)]
    pub const fn eccr3(&self) -> &ECCR3 {
        &self.eccr3
    }
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    #[inline(always)]
    pub const fn pcr4(&self) -> &PCR4 {
        &self.pcr4
    }
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    #[inline(always)]
    pub const fn sr4(&self) -> &SR4 {
        &self.sr4
    }
    #[doc = "0xa8 - Common memory space timing register 4"]
    #[inline(always)]
    pub const fn pmem4(&self) -> &PMEM4 {
        &self.pmem4
    }
    #[doc = "0xac - Attribute memory space timing register 4"]
    #[inline(always)]
    pub const fn patt4(&self) -> &PATT4 {
        &self.patt4
    }
    #[doc = "0xb0 - I/O space timing register 4"]
    #[inline(always)]
    pub const fn pio4(&self) -> &PIO4 {
        &self.pio4
    }
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR1 {
        &self.bwtr1
    }
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR2 {
        &self.bwtr2
    }
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR3 {
        &self.bwtr3
    }
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR4 {
        &self.bwtr4
    }
}
#[doc = "BCR1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr1`]
module"]
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR1 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr1`]
module"]
pub type BTR1 = crate::Reg<btr1::BTR1rs>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "BCR2 (rw) register accessor: SRAM/NOR-Flash chip-select control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr2`]
module"]
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "BTR2 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr2`]
module"]
pub type BTR2 = crate::Reg<btr2::BTR2rs>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "BCR3 (rw) register accessor: SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr3`]
module"]
pub type BCR3 = crate::Reg<bcr3::BCR3rs>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "BTR3 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr3`]
module"]
pub type BTR3 = crate::Reg<btr3::BTR3rs>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "BCR4 (rw) register accessor: SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr4`]
module"]
pub type BCR4 = crate::Reg<bcr4::BCR4rs>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "BTR4 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr4`]
module"]
pub type BTR4 = crate::Reg<btr4::BTR4rs>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PCR2 (rw) register accessor: PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr2`]
module"]
pub type PCR2 = crate::Reg<pcr2::PCR2rs>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod pcr2;
#[doc = "SR2 (rw) register accessor: FIFO status and interrupt register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2rs>;
#[doc = "FIFO status and interrupt register 2"]
pub mod sr2;
#[doc = "PMEM2 (rw) register accessor: Common memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmem2`]
module"]
pub type PMEM2 = crate::Reg<pmem2::PMEM2rs>;
#[doc = "Common memory space timing register 2"]
pub mod pmem2;
#[doc = "PATT2 (rw) register accessor: Attribute memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt2`]
module"]
pub type PATT2 = crate::Reg<patt2::PATT2rs>;
#[doc = "Attribute memory space timing register 2"]
pub mod patt2;
#[doc = "ECCR2 (r) register accessor: ECC result register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr2`]
module"]
pub type ECCR2 = crate::Reg<eccr2::ECCR2rs>;
#[doc = "ECC result register 2"]
pub mod eccr2;
#[doc = "PCR3 (rw) register accessor: PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr3`]
module"]
pub type PCR3 = crate::Reg<pcr3::PCR3rs>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr3;
#[doc = "SR3 (rw) register accessor: FIFO status and interrupt register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr3`]
module"]
pub type SR3 = crate::Reg<sr3::SR3rs>;
#[doc = "FIFO status and interrupt register 3"]
pub mod sr3;
#[doc = "PMEM3 (rw) register accessor: Common memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmem3`]
module"]
pub type PMEM3 = crate::Reg<pmem3::PMEM3rs>;
#[doc = "Common memory space timing register 3"]
pub mod pmem3;
#[doc = "PATT3 (rw) register accessor: Attribute memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt3`]
module"]
pub type PATT3 = crate::Reg<patt3::PATT3rs>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt3;
#[doc = "ECCR3 (r) register accessor: ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr3`]
module"]
pub type ECCR3 = crate::Reg<eccr3::ECCR3rs>;
#[doc = "ECC result register 3"]
pub mod eccr3;
#[doc = "PCR4 (rw) register accessor: PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr4`]
module"]
pub type PCR4 = crate::Reg<pcr4::PCR4rs>;
#[doc = "PC Card/NAND Flash control register 4"]
pub mod pcr4;
#[doc = "SR4 (rw) register accessor: FIFO status and interrupt register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr4`]
module"]
pub type SR4 = crate::Reg<sr4::SR4rs>;
#[doc = "FIFO status and interrupt register 4"]
pub mod sr4;
#[doc = "PMEM4 (rw) register accessor: Common memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmem4`]
module"]
pub type PMEM4 = crate::Reg<pmem4::PMEM4rs>;
#[doc = "Common memory space timing register 4"]
pub mod pmem4;
#[doc = "PATT4 (rw) register accessor: Attribute memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt4`]
module"]
pub type PATT4 = crate::Reg<patt4::PATT4rs>;
#[doc = "Attribute memory space timing register 4"]
pub mod patt4;
#[doc = "PIO4 (rw) register accessor: I/O space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pio4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pio4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pio4`]
module"]
pub type PIO4 = crate::Reg<pio4::PIO4rs>;
#[doc = "I/O space timing register 4"]
pub mod pio4;
#[doc = "BWTR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr1`]
module"]
pub type BWTR1 = crate::Reg<bwtr1::BWTR1rs>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "BWTR2 (rw) register accessor: SRAM/NOR-Flash write timing registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr2`]
module"]
pub type BWTR2 = crate::Reg<bwtr2::BWTR2rs>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "BWTR3 (rw) register accessor: SRAM/NOR-Flash write timing registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr3`]
module"]
pub type BWTR3 = crate::Reg<bwtr3::BWTR3rs>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "BWTR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr4`]
module"]
pub type BWTR4 = crate::Reg<bwtr4::BWTR4rs>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
