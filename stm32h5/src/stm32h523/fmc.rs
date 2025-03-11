#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bcr1: BCR1,
    btr1: BTR1,
    bcr2: BCR2,
    btr2: BTR2,
    bcr3: BCR3,
    btr3: BTR3,
    bcr4: BCR4,
    btr4: BTR4,
    pcscntr: PCSCNTR,
    _reserved9: [u8; 0x5c],
    pcr: PCR,
    sr: SR,
    pmem: PMEM,
    patt: PATT,
    _reserved13: [u8; 0x04],
    eccr: ECCR,
    _reserved14: [u8; 0x6c],
    bwtr1: BWTR1,
    _reserved15: [u8; 0x04],
    bwtr2: BWTR2,
    _reserved16: [u8; 0x04],
    bwtr3: BWTR3,
    _reserved17: [u8; 0x04],
    bwtr4: BWTR4,
    _reserved18: [u8; 0x20],
    sdcr1: SDCR1,
    sdcr2: SDCR2,
    sdtr1: SDTR1,
    sdtr2: SDTR2,
    sdcmr: SDCMR,
    sdrtr: SDRTR,
    sdsr: SDSR,
}
impl RegisterBlock {
    ///0x00 - SRAM/NOR-flash chip-select control register for bank 1
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x04 - SRAM/NOR-flash chip-select timing register for bank 1
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR1 {
        &self.btr1
    }
    ///0x08 - SRAM/NOR-flash chip-select control register for bank 2
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    ///0x0c - SRAM/NOR-flash chip-select timing register for bank 2
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR2 {
        &self.btr2
    }
    ///0x10 - SRAM/NOR-flash chip-select control register for bank 3
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR3 {
        &self.bcr3
    }
    ///0x14 - SRAM/NOR-flash chip-select timing register for bank 3
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR3 {
        &self.btr3
    }
    ///0x18 - SRAM/NOR-flash chip-select control register for bank 4
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR4 {
        &self.bcr4
    }
    ///0x1c - SRAM/NOR-flash chip-select timing register for bank 4
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR4 {
        &self.btr4
    }
    ///0x20 - PSRAM chip select counter register
    #[inline(always)]
    pub const fn pcscntr(&self) -> &PCSCNTR {
        &self.pcscntr
    }
    ///0x80 - NAND flash control registers
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    ///0x84 - FIFO status and interrupt register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x88 - Common memory space timing register
    #[inline(always)]
    pub const fn pmem(&self) -> &PMEM {
        &self.pmem
    }
    ///0x8c - Attribute memory space timing register
    #[inline(always)]
    pub const fn patt(&self) -> &PATT {
        &self.patt
    }
    ///0x94 - ECC result registers
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x104 - SRAM/NOR-flash write timing registers 1
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR1 {
        &self.bwtr1
    }
    ///0x10c - SRAM/NOR-flash write timing registers 2
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR2 {
        &self.bwtr2
    }
    ///0x114 - SRAM/NOR-flash write timing registers 3
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR3 {
        &self.bwtr3
    }
    ///0x11c - SRAM/NOR-flash write timing registers 4
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR4 {
        &self.bwtr4
    }
    ///0x140 - SDRAM control registers 1,2
    #[inline(always)]
    pub const fn sdcr1(&self) -> &SDCR1 {
        &self.sdcr1
    }
    ///0x144 - SDRAM control registers 1,2
    #[inline(always)]
    pub const fn sdcr2(&self) -> &SDCR2 {
        &self.sdcr2
    }
    ///0x148 - SDRAM timing registers 1,2
    #[inline(always)]
    pub const fn sdtr1(&self) -> &SDTR1 {
        &self.sdtr1
    }
    ///0x14c - SDRAM timing registers 1,2
    #[inline(always)]
    pub const fn sdtr2(&self) -> &SDTR2 {
        &self.sdtr2
    }
    ///0x150 - SDRAM Command Mode register
    #[inline(always)]
    pub const fn sdcmr(&self) -> &SDCMR {
        &self.sdcmr
    }
    ///0x154 - SDRAM refresh timer register
    #[inline(always)]
    pub const fn sdrtr(&self) -> &SDRTR {
        &self.sdrtr
    }
    ///0x158 - SDRAM status register
    #[inline(always)]
    pub const fn sdsr(&self) -> &SDSR {
        &self.sdsr
    }
}
/**BCR1 (rw) register accessor: SRAM/NOR-flash chip-select control register for bank 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///SRAM/NOR-flash chip-select control register for bank 1
pub mod bcr1;
/**BTR1 (rw) register accessor: SRAM/NOR-flash chip-select timing register for bank 1

You can [`read`](crate::Reg::read) this register and get [`btr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BTR1)

For information about available fields see [`mod@btr1`] module*/
pub type BTR1 = crate::Reg<btr1::BTR1rs>;
///SRAM/NOR-flash chip-select timing register for bank 1
pub mod btr1;
/**BCR2 (rw) register accessor: SRAM/NOR-flash chip-select control register for bank 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BCR2)

For information about available fields see [`mod@bcr2`] module*/
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
///SRAM/NOR-flash chip-select control register for bank 2
pub mod bcr2;
/**BTR2 (rw) register accessor: SRAM/NOR-flash chip-select timing register for bank 2

You can [`read`](crate::Reg::read) this register and get [`btr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BTR2)

For information about available fields see [`mod@btr2`] module*/
pub type BTR2 = crate::Reg<btr2::BTR2rs>;
///SRAM/NOR-flash chip-select timing register for bank 2
pub mod btr2;
/**BCR3 (rw) register accessor: SRAM/NOR-flash chip-select control register for bank 3

You can [`read`](crate::Reg::read) this register and get [`bcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BCR3)

For information about available fields see [`mod@bcr3`] module*/
pub type BCR3 = crate::Reg<bcr3::BCR3rs>;
///SRAM/NOR-flash chip-select control register for bank 3
pub mod bcr3;
/**BTR3 (rw) register accessor: SRAM/NOR-flash chip-select timing register for bank 3

You can [`read`](crate::Reg::read) this register and get [`btr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BTR3)

For information about available fields see [`mod@btr3`] module*/
pub type BTR3 = crate::Reg<btr3::BTR3rs>;
///SRAM/NOR-flash chip-select timing register for bank 3
pub mod btr3;
/**BCR4 (rw) register accessor: SRAM/NOR-flash chip-select control register for bank 4

You can [`read`](crate::Reg::read) this register and get [`bcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BCR4)

For information about available fields see [`mod@bcr4`] module*/
pub type BCR4 = crate::Reg<bcr4::BCR4rs>;
///SRAM/NOR-flash chip-select control register for bank 4
pub mod bcr4;
/**BTR4 (rw) register accessor: SRAM/NOR-flash chip-select timing register for bank 4

You can [`read`](crate::Reg::read) this register and get [`btr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BTR4)

For information about available fields see [`mod@btr4`] module*/
pub type BTR4 = crate::Reg<btr4::BTR4rs>;
///SRAM/NOR-flash chip-select timing register for bank 4
pub mod btr4;
/**PCSCNTR (rw) register accessor: PSRAM chip select counter register

You can [`read`](crate::Reg::read) this register and get [`pcscntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcscntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:PCSCNTR)

For information about available fields see [`mod@pcscntr`] module*/
pub type PCSCNTR = crate::Reg<pcscntr::PCSCNTRrs>;
///PSRAM chip select counter register
pub mod pcscntr;
/**PCR (rw) register accessor: NAND flash control registers

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///NAND flash control registers
pub mod pcr;
/**SR (rw) register accessor: FIFO status and interrupt register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FIFO status and interrupt register
pub mod sr;
/**PMEM (rw) register accessor: Common memory space timing register

You can [`read`](crate::Reg::read) this register and get [`pmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:PMEM)

For information about available fields see [`mod@pmem`] module*/
pub type PMEM = crate::Reg<pmem::PMEMrs>;
///Common memory space timing register
pub mod pmem;
/**PATT (rw) register accessor: Attribute memory space timing register

You can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:PATT)

For information about available fields see [`mod@patt`] module*/
pub type PATT = crate::Reg<patt::PATTrs>;
///Attribute memory space timing register
pub mod patt;
/**ECCR (r) register accessor: ECC result registers

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///ECC result registers
pub mod eccr;
/**BWTR1 (rw) register accessor: SRAM/NOR-flash write timing registers 1

You can [`read`](crate::Reg::read) this register and get [`bwtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BWTR1)

For information about available fields see [`mod@bwtr1`] module*/
pub type BWTR1 = crate::Reg<bwtr1::BWTR1rs>;
///SRAM/NOR-flash write timing registers 1
pub mod bwtr1;
/**BWTR2 (rw) register accessor: SRAM/NOR-flash write timing registers 2

You can [`read`](crate::Reg::read) this register and get [`bwtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BWTR2)

For information about available fields see [`mod@bwtr2`] module*/
pub type BWTR2 = crate::Reg<bwtr2::BWTR2rs>;
///SRAM/NOR-flash write timing registers 2
pub mod bwtr2;
/**BWTR3 (rw) register accessor: SRAM/NOR-flash write timing registers 3

You can [`read`](crate::Reg::read) this register and get [`bwtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BWTR3)

For information about available fields see [`mod@bwtr3`] module*/
pub type BWTR3 = crate::Reg<bwtr3::BWTR3rs>;
///SRAM/NOR-flash write timing registers 3
pub mod bwtr3;
/**BWTR4 (rw) register accessor: SRAM/NOR-flash write timing registers 4

You can [`read`](crate::Reg::read) this register and get [`bwtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:BWTR4)

For information about available fields see [`mod@bwtr4`] module*/
pub type BWTR4 = crate::Reg<bwtr4::BWTR4rs>;
///SRAM/NOR-flash write timing registers 4
pub mod bwtr4;
/**SDCR1 (rw) register accessor: SDRAM control registers 1,2

You can [`read`](crate::Reg::read) this register and get [`sdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDCR1)

For information about available fields see [`mod@sdcr1`] module*/
pub type SDCR1 = crate::Reg<sdcr1::SDCR1rs>;
///SDRAM control registers 1,2
pub mod sdcr1;
/**SDCR2 (rw) register accessor: SDRAM control registers 1,2

You can [`read`](crate::Reg::read) this register and get [`sdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDCR2)

For information about available fields see [`mod@sdcr2`] module*/
pub type SDCR2 = crate::Reg<sdcr2::SDCR2rs>;
///SDRAM control registers 1,2
pub mod sdcr2;
/**SDTR1 (rw) register accessor: SDRAM timing registers 1,2

You can [`read`](crate::Reg::read) this register and get [`sdtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDTR1)

For information about available fields see [`mod@sdtr1`] module*/
pub type SDTR1 = crate::Reg<sdtr1::SDTR1rs>;
///SDRAM timing registers 1,2
pub mod sdtr1;
/**SDTR2 (rw) register accessor: SDRAM timing registers 1,2

You can [`read`](crate::Reg::read) this register and get [`sdtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDTR2)

For information about available fields see [`mod@sdtr2`] module*/
pub type SDTR2 = crate::Reg<sdtr2::SDTR2rs>;
///SDRAM timing registers 1,2
pub mod sdtr2;
/**SDCMR (rw) register accessor: SDRAM Command Mode register

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDCMR)

For information about available fields see [`mod@sdcmr`] module*/
pub type SDCMR = crate::Reg<sdcmr::SDCMRrs>;
///SDRAM Command Mode register
pub mod sdcmr;
/**SDRTR (rw) register accessor: SDRAM refresh timer register

You can [`read`](crate::Reg::read) this register and get [`sdrtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDRTR)

For information about available fields see [`mod@sdrtr`] module*/
pub type SDRTR = crate::Reg<sdrtr::SDRTRrs>;
///SDRAM refresh timer register
pub mod sdrtr;
/**SDSR (r) register accessor: SDRAM status register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDSR)

For information about available fields see [`mod@sdsr`] module*/
pub type SDSR = crate::Reg<sdsr::SDSRrs>;
///SDRAM status register
pub mod sdsr;
