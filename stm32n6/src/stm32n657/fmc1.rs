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
    cfgr: CFGR,
    _reserved9: [u8; 0x5c],
    pcr: PCR,
    sr: SR,
    pmem: PMEM,
    patt: PATT,
    hpr: HPR,
    heccr: HECCR,
    _reserved15: [u8; 0x6c],
    bwtr1: BWTR1,
    _reserved16: [u8; 0x04],
    bwtr2: BWTR2,
    _reserved17: [u8; 0x04],
    bwtr3: BWTR3,
    _reserved18: [u8; 0x04],
    bwtr4: BWTR4,
    _reserved19: [u8; 0x20],
    sdcr1: SDCR1,
    sdcr2: SDCR2,
    sdtr: SDTR,
    _reserved22: [u8; 0x04],
    sdcmr: SDCMR,
    sdrtr: SDRTR,
    sdsr: SDSR,
    _reserved25: [u8; 0x24],
    ier: IER,
    isr: ISR,
    icr: ICR,
    _reserved28: [u8; 0x74],
    csqcr: CSQCR,
    csqcfgr1: CSQCFGR1,
    csqcfgr2: CSQCFGR2,
    csqcfgr3: CSQCFGR3,
    csqar1: CSQAR1,
    csqar2: CSQAR2,
    _reserved34: [u8; 0x08],
    csqier: CSQIER,
    csqisr: CSQISR,
    csqicr: CSQICR,
    _reserved37: [u8; 0x04],
    csqemsr: CSQEMSR,
    _reserved38: [u8; 0x1c],
    bchier: BCHIER,
    bchisr: BCHISR,
    bchicr: BCHICR,
    _reserved41: [u8; 0x04],
    bchpbr1: BCHPBR1,
    bchpbr2: BCHPBR2,
    bchpbr3: BCHPBR3,
    bchpbr4: BCHPBR4,
    _reserved45: [u8; 0x0c],
    bchdsr0: BCHDSR0,
    bchdsr1: BCHDSR1,
    bchdsr2: BCHDSR2,
    bchdsr3: BCHDSR3,
    bchdsr4: BCHDSR4,
}
impl RegisterBlock {
    ///0x00 - SRAM/NOR Flash chip-select control register for memory region 1
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x04 - SRAM/NOR Flash chip-select timing registers for memory region 1
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR1 {
        &self.btr1
    }
    ///0x08 - SRAM/NOR Flash chip-select control register for memory region 2
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    ///0x0c - SRAM/NOR Flash chip-select timing registers for memory region 2
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR2 {
        &self.btr2
    }
    ///0x10 - SRAM/NOR Flash chip-select control register for memory region 3
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR3 {
        &self.bcr3
    }
    ///0x14 - SRAM/NOR Flash chip-select timing registers for memory region 3
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR3 {
        &self.btr3
    }
    ///0x18 - SRAM/NOR Flash chip-select control register for memory region 4
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR4 {
        &self.bcr4
    }
    ///0x1c - SRAM/NOR Flash chip-select timing registers for memory region 4
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR4 {
        &self.btr4
    }
    ///0x20 - FMC common configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x80 - NAND Flash programmable control register
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    ///0x84 - FMC status register
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    ///0x88 - FMC common memory space timing register
    #[inline(always)]
    pub const fn pmem(&self) -> &PMEM {
        &self.pmem
    }
    ///0x8c - FMC attribute memory space timing registers
    #[inline(always)]
    pub const fn patt(&self) -> &PATT {
        &self.patt
    }
    ///0x90 - FMC Hamming parity result registers
    #[inline(always)]
    pub const fn hpr(&self) -> &HPR {
        &self.hpr
    }
    ///0x94 - FMC Hamming code ECC result register
    #[inline(always)]
    pub const fn heccr(&self) -> &HECCR {
        &self.heccr
    }
    ///0x104 - SRAM/NOR-Flash write timing registers for memory region 1
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR1 {
        &self.bwtr1
    }
    ///0x10c - SRAM/NOR-Flash write timing registers for memory region 2
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR2 {
        &self.bwtr2
    }
    ///0x114 - SRAM/NOR-Flash write timing registers for memory region 3
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR3 {
        &self.bwtr3
    }
    ///0x11c - SRAM/NOR-Flash write timing registers for memory region 4
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR4 {
        &self.bwtr4
    }
    ///0x140 - SDRAM control registers for SDRAM device 1
    #[inline(always)]
    pub const fn sdcr1(&self) -> &SDCR1 {
        &self.sdcr1
    }
    ///0x144 - SDRAM control registers for SDRAM device 2
    #[inline(always)]
    pub const fn sdcr2(&self) -> &SDCR2 {
        &self.sdcr2
    }
    ///0x148 - SDRAM timing register
    #[inline(always)]
    pub const fn sdtr(&self) -> &SDTR {
        &self.sdtr
    }
    ///0x150 - SDRAM command mode register
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
    ///0x180 - FMC NAND interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x184 - FMC controller interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x188 - FMC NAND controller interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x200 - FMC NAND command sequencer control register
    #[inline(always)]
    pub const fn csqcr(&self) -> &CSQCR {
        &self.csqcr
    }
    ///0x204 - FMC NAND command sequencer configuration register 1
    #[inline(always)]
    pub const fn csqcfgr1(&self) -> &CSQCFGR1 {
        &self.csqcfgr1
    }
    ///0x208 - FMC NAND command sequencer configuration register 2
    #[inline(always)]
    pub const fn csqcfgr2(&self) -> &CSQCFGR2 {
        &self.csqcfgr2
    }
    ///0x20c - FMC NAND sequencer configuration register 3
    #[inline(always)]
    pub const fn csqcfgr3(&self) -> &CSQCFGR3 {
        &self.csqcfgr3
    }
    ///0x210 - FMC NAND command sequencer address register 1
    #[inline(always)]
    pub const fn csqar1(&self) -> &CSQAR1 {
        &self.csqar1
    }
    ///0x214 - FMC NAND command sequencer address register 2
    #[inline(always)]
    pub const fn csqar2(&self) -> &CSQAR2 {
        &self.csqar2
    }
    ///0x220 - FMC NAND command sequencer interrupt enable register
    #[inline(always)]
    pub const fn csqier(&self) -> &CSQIER {
        &self.csqier
    }
    ///0x224 - FMC NAND command sequencer interrupt status register
    #[inline(always)]
    pub const fn csqisr(&self) -> &CSQISR {
        &self.csqisr
    }
    ///0x228 - FMC NAND command sequencer interrupt clear register
    #[inline(always)]
    pub const fn csqicr(&self) -> &CSQICR {
        &self.csqicr
    }
    ///0x230 - FMC command sequencer error mapping status register
    #[inline(always)]
    pub const fn csqemsr(&self) -> &CSQEMSR {
        &self.csqemsr
    }
    ///0x250 - FMC BCH interrupt enable register
    #[inline(always)]
    pub const fn bchier(&self) -> &BCHIER {
        &self.bchier
    }
    ///0x254 - FMC BCH interrupt and status register
    #[inline(always)]
    pub const fn bchisr(&self) -> &BCHISR {
        &self.bchisr
    }
    ///0x258 - FMC BCH interrupt clear register
    #[inline(always)]
    pub const fn bchicr(&self) -> &BCHICR {
        &self.bchicr
    }
    ///0x260 - FMC BCH parity bits register 1
    #[inline(always)]
    pub const fn bchpbr1(&self) -> &BCHPBR1 {
        &self.bchpbr1
    }
    ///0x264 - FMC BCH parity bits register 2
    #[inline(always)]
    pub const fn bchpbr2(&self) -> &BCHPBR2 {
        &self.bchpbr2
    }
    ///0x268 - FMC BCH parity bits register 3
    #[inline(always)]
    pub const fn bchpbr3(&self) -> &BCHPBR3 {
        &self.bchpbr3
    }
    ///0x26c - FMC BCH parity bits register 4
    #[inline(always)]
    pub const fn bchpbr4(&self) -> &BCHPBR4 {
        &self.bchpbr4
    }
    ///0x27c - FMC BCH decoder status register 0
    #[inline(always)]
    pub const fn bchdsr0(&self) -> &BCHDSR0 {
        &self.bchdsr0
    }
    ///0x280 - FMC BCH decoder status register for memory region 1
    #[inline(always)]
    pub const fn bchdsr1(&self) -> &BCHDSR1 {
        &self.bchdsr1
    }
    ///0x284 - FMC BCH decoder status register for memory region 2
    #[inline(always)]
    pub const fn bchdsr2(&self) -> &BCHDSR2 {
        &self.bchdsr2
    }
    ///0x288 - FMC BCH decoder status register for memory region 3
    #[inline(always)]
    pub const fn bchdsr3(&self) -> &BCHDSR3 {
        &self.bchdsr3
    }
    ///0x28c - FMC BCH decoder status register for memory region 4
    #[inline(always)]
    pub const fn bchdsr4(&self) -> &BCHDSR4 {
        &self.bchdsr4
    }
}
/**BCR1 (rw) register accessor: SRAM/NOR Flash chip-select control register for memory region 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///SRAM/NOR Flash chip-select control register for memory region 1
pub mod bcr1;
/**BTR1 (rw) register accessor: SRAM/NOR Flash chip-select timing registers for memory region 1

You can [`read`](crate::Reg::read) this register and get [`btr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BTR1)

For information about available fields see [`mod@btr1`] module*/
pub type BTR1 = crate::Reg<btr1::BTR1rs>;
///SRAM/NOR Flash chip-select timing registers for memory region 1
pub mod btr1;
/**BCR2 (rw) register accessor: SRAM/NOR Flash chip-select control register for memory region 2

You can [`read`](crate::Reg::read) this register and get [`bcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCR2)

For information about available fields see [`mod@bcr2`] module*/
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
///SRAM/NOR Flash chip-select control register for memory region 2
pub mod bcr2;
/**BTR2 (rw) register accessor: SRAM/NOR Flash chip-select timing registers for memory region 2

You can [`read`](crate::Reg::read) this register and get [`btr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BTR2)

For information about available fields see [`mod@btr2`] module*/
pub type BTR2 = crate::Reg<btr2::BTR2rs>;
///SRAM/NOR Flash chip-select timing registers for memory region 2
pub mod btr2;
/**BCR3 (rw) register accessor: SRAM/NOR Flash chip-select control register for memory region 3

You can [`read`](crate::Reg::read) this register and get [`bcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCR3)

For information about available fields see [`mod@bcr3`] module*/
pub type BCR3 = crate::Reg<bcr3::BCR3rs>;
///SRAM/NOR Flash chip-select control register for memory region 3
pub mod bcr3;
/**BTR3 (rw) register accessor: SRAM/NOR Flash chip-select timing registers for memory region 3

You can [`read`](crate::Reg::read) this register and get [`btr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BTR3)

For information about available fields see [`mod@btr3`] module*/
pub type BTR3 = crate::Reg<btr3::BTR3rs>;
///SRAM/NOR Flash chip-select timing registers for memory region 3
pub mod btr3;
/**BCR4 (rw) register accessor: SRAM/NOR Flash chip-select control register for memory region 4

You can [`read`](crate::Reg::read) this register and get [`bcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCR4)

For information about available fields see [`mod@bcr4`] module*/
pub type BCR4 = crate::Reg<bcr4::BCR4rs>;
///SRAM/NOR Flash chip-select control register for memory region 4
pub mod bcr4;
/**BTR4 (rw) register accessor: SRAM/NOR Flash chip-select timing registers for memory region 4

You can [`read`](crate::Reg::read) this register and get [`btr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BTR4)

For information about available fields see [`mod@btr4`] module*/
pub type BTR4 = crate::Reg<btr4::BTR4rs>;
///SRAM/NOR Flash chip-select timing registers for memory region 4
pub mod btr4;
/**CFGR (rw) register accessor: FMC common configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///FMC common configuration register
pub mod cfgr;
/**PCR (rw) register accessor: NAND Flash programmable control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///NAND Flash programmable control register
pub mod pcr;
/**SR (r) register accessor: FMC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FMC status register
pub mod sr;
/**PMEM (rw) register accessor: FMC common memory space timing register

You can [`read`](crate::Reg::read) this register and get [`pmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:PMEM)

For information about available fields see [`mod@pmem`] module*/
pub type PMEM = crate::Reg<pmem::PMEMrs>;
///FMC common memory space timing register
pub mod pmem;
/**PATT (rw) register accessor: FMC attribute memory space timing registers

You can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:PATT)

For information about available fields see [`mod@patt`] module*/
pub type PATT = crate::Reg<patt::PATTrs>;
///FMC attribute memory space timing registers
pub mod patt;
/**HPR (r) register accessor: FMC Hamming parity result registers

You can [`read`](crate::Reg::read) this register and get [`hpr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:HPR)

For information about available fields see [`mod@hpr`] module*/
pub type HPR = crate::Reg<hpr::HPRrs>;
///FMC Hamming parity result registers
pub mod hpr;
/**HECCR (r) register accessor: FMC Hamming code ECC result register

You can [`read`](crate::Reg::read) this register and get [`heccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:HECCR)

For information about available fields see [`mod@heccr`] module*/
pub type HECCR = crate::Reg<heccr::HECCRrs>;
///FMC Hamming code ECC result register
pub mod heccr;
/**BWTR1 (rw) register accessor: SRAM/NOR-Flash write timing registers for memory region 1

You can [`read`](crate::Reg::read) this register and get [`bwtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BWTR1)

For information about available fields see [`mod@bwtr1`] module*/
pub type BWTR1 = crate::Reg<bwtr1::BWTR1rs>;
///SRAM/NOR-Flash write timing registers for memory region 1
pub mod bwtr1;
/**BWTR2 (rw) register accessor: SRAM/NOR-Flash write timing registers for memory region 2

You can [`read`](crate::Reg::read) this register and get [`bwtr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BWTR2)

For information about available fields see [`mod@bwtr2`] module*/
pub type BWTR2 = crate::Reg<bwtr2::BWTR2rs>;
///SRAM/NOR-Flash write timing registers for memory region 2
pub mod bwtr2;
/**BWTR3 (rw) register accessor: SRAM/NOR-Flash write timing registers for memory region 3

You can [`read`](crate::Reg::read) this register and get [`bwtr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BWTR3)

For information about available fields see [`mod@bwtr3`] module*/
pub type BWTR3 = crate::Reg<bwtr3::BWTR3rs>;
///SRAM/NOR-Flash write timing registers for memory region 3
pub mod bwtr3;
/**BWTR4 (rw) register accessor: SRAM/NOR-Flash write timing registers for memory region 4

You can [`read`](crate::Reg::read) this register and get [`bwtr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BWTR4)

For information about available fields see [`mod@bwtr4`] module*/
pub type BWTR4 = crate::Reg<bwtr4::BWTR4rs>;
///SRAM/NOR-Flash write timing registers for memory region 4
pub mod bwtr4;
/**SDCR1 (rw) register accessor: SDRAM control registers for SDRAM device 1

You can [`read`](crate::Reg::read) this register and get [`sdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SDCR1)

For information about available fields see [`mod@sdcr1`] module*/
pub type SDCR1 = crate::Reg<sdcr1::SDCR1rs>;
///SDRAM control registers for SDRAM device 1
pub mod sdcr1;
/**SDCR2 (rw) register accessor: SDRAM control registers for SDRAM device 2

You can [`read`](crate::Reg::read) this register and get [`sdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SDCR2)

For information about available fields see [`mod@sdcr2`] module*/
pub type SDCR2 = crate::Reg<sdcr2::SDCR2rs>;
///SDRAM control registers for SDRAM device 2
pub mod sdcr2;
/**SDTR (rw) register accessor: SDRAM timing register

You can [`read`](crate::Reg::read) this register and get [`sdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SDTR)

For information about available fields see [`mod@sdtr`] module*/
pub type SDTR = crate::Reg<sdtr::SDTRrs>;
///SDRAM timing register
pub mod sdtr;
/**SDCMR (rw) register accessor: SDRAM command mode register

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SDCMR)

For information about available fields see [`mod@sdcmr`] module*/
pub type SDCMR = crate::Reg<sdcmr::SDCMRrs>;
///SDRAM command mode register
pub mod sdcmr;
/**SDRTR (rw) register accessor: SDRAM refresh timer register

You can [`read`](crate::Reg::read) this register and get [`sdrtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SDRTR)

For information about available fields see [`mod@sdrtr`] module*/
pub type SDRTR = crate::Reg<sdrtr::SDRTRrs>;
///SDRAM refresh timer register
pub mod sdrtr;
/**SDSR (r) register accessor: SDRAM status register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:SDSR)

For information about available fields see [`mod@sdsr`] module*/
pub type SDSR = crate::Reg<sdsr::SDSRrs>;
///SDRAM status register
pub mod sdsr;
/**IER (rw) register accessor: FMC NAND interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///FMC NAND interrupt enable register
pub mod ier;
/**ISR (r) register accessor: FMC controller interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///FMC controller interrupt status register
pub mod isr;
/**ICR (w) register accessor: FMC NAND controller interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///FMC NAND controller interrupt clear register
pub mod icr;
/**CSQCR (w) register accessor: FMC NAND command sequencer control register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQCR)

For information about available fields see [`mod@csqcr`] module*/
pub type CSQCR = crate::Reg<csqcr::CSQCRrs>;
///FMC NAND command sequencer control register
pub mod csqcr;
/**CSQCFGR1 (rw) register accessor: FMC NAND command sequencer configuration register 1

You can [`read`](crate::Reg::read) this register and get [`csqcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQCFGR1)

For information about available fields see [`mod@csqcfgr1`] module*/
pub type CSQCFGR1 = crate::Reg<csqcfgr1::CSQCFGR1rs>;
///FMC NAND command sequencer configuration register 1
pub mod csqcfgr1;
/**CSQCFGR2 (rw) register accessor: FMC NAND command sequencer configuration register 2

You can [`read`](crate::Reg::read) this register and get [`csqcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQCFGR2)

For information about available fields see [`mod@csqcfgr2`] module*/
pub type CSQCFGR2 = crate::Reg<csqcfgr2::CSQCFGR2rs>;
///FMC NAND command sequencer configuration register 2
pub mod csqcfgr2;
/**CSQCFGR3 (rw) register accessor: FMC NAND sequencer configuration register 3

You can [`read`](crate::Reg::read) this register and get [`csqcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQCFGR3)

For information about available fields see [`mod@csqcfgr3`] module*/
pub type CSQCFGR3 = crate::Reg<csqcfgr3::CSQCFGR3rs>;
///FMC NAND sequencer configuration register 3
pub mod csqcfgr3;
/**CSQAR1 (rw) register accessor: FMC NAND command sequencer address register 1

You can [`read`](crate::Reg::read) this register and get [`csqar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQAR1)

For information about available fields see [`mod@csqar1`] module*/
pub type CSQAR1 = crate::Reg<csqar1::CSQAR1rs>;
///FMC NAND command sequencer address register 1
pub mod csqar1;
/**CSQAR2 (rw) register accessor: FMC NAND command sequencer address register 2

You can [`read`](crate::Reg::read) this register and get [`csqar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQAR2)

For information about available fields see [`mod@csqar2`] module*/
pub type CSQAR2 = crate::Reg<csqar2::CSQAR2rs>;
///FMC NAND command sequencer address register 2
pub mod csqar2;
/**CSQIER (rw) register accessor: FMC NAND command sequencer interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`csqier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQIER)

For information about available fields see [`mod@csqier`] module*/
pub type CSQIER = crate::Reg<csqier::CSQIERrs>;
///FMC NAND command sequencer interrupt enable register
pub mod csqier;
/**CSQISR (rw) register accessor: FMC NAND command sequencer interrupt status register

You can [`read`](crate::Reg::read) this register and get [`csqisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQISR)

For information about available fields see [`mod@csqisr`] module*/
pub type CSQISR = crate::Reg<csqisr::CSQISRrs>;
///FMC NAND command sequencer interrupt status register
pub mod csqisr;
/**CSQICR (w) register accessor: FMC NAND command sequencer interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQICR)

For information about available fields see [`mod@csqicr`] module*/
pub type CSQICR = crate::Reg<csqicr::CSQICRrs>;
///FMC NAND command sequencer interrupt clear register
pub mod csqicr;
/**CSQEMSR (r) register accessor: FMC command sequencer error mapping status register

You can [`read`](crate::Reg::read) this register and get [`csqemsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CSQEMSR)

For information about available fields see [`mod@csqemsr`] module*/
pub type CSQEMSR = crate::Reg<csqemsr::CSQEMSRrs>;
///FMC command sequencer error mapping status register
pub mod csqemsr;
/**BCHIER (rw) register accessor: FMC BCH interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`bchier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bchier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHIER)

For information about available fields see [`mod@bchier`] module*/
pub type BCHIER = crate::Reg<bchier::BCHIERrs>;
///FMC BCH interrupt enable register
pub mod bchier;
/**BCHISR (r) register accessor: FMC BCH interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`bchisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHISR)

For information about available fields see [`mod@bchisr`] module*/
pub type BCHISR = crate::Reg<bchisr::BCHISRrs>;
///FMC BCH interrupt and status register
pub mod bchisr;
/**BCHICR (w) register accessor: FMC BCH interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bchicr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHICR)

For information about available fields see [`mod@bchicr`] module*/
pub type BCHICR = crate::Reg<bchicr::BCHICRrs>;
///FMC BCH interrupt clear register
pub mod bchicr;
/**BCHPBR1 (r) register accessor: FMC BCH parity bits register 1

You can [`read`](crate::Reg::read) this register and get [`bchpbr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHPBR1)

For information about available fields see [`mod@bchpbr1`] module*/
pub type BCHPBR1 = crate::Reg<bchpbr1::BCHPBR1rs>;
///FMC BCH parity bits register 1
pub mod bchpbr1;
/**BCHPBR2 (r) register accessor: FMC BCH parity bits register 2

You can [`read`](crate::Reg::read) this register and get [`bchpbr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHPBR2)

For information about available fields see [`mod@bchpbr2`] module*/
pub type BCHPBR2 = crate::Reg<bchpbr2::BCHPBR2rs>;
///FMC BCH parity bits register 2
pub mod bchpbr2;
/**BCHPBR3 (r) register accessor: FMC BCH parity bits register 3

You can [`read`](crate::Reg::read) this register and get [`bchpbr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHPBR3)

For information about available fields see [`mod@bchpbr3`] module*/
pub type BCHPBR3 = crate::Reg<bchpbr3::BCHPBR3rs>;
///FMC BCH parity bits register 3
pub mod bchpbr3;
/**BCHPBR4 (r) register accessor: FMC BCH parity bits register 4

You can [`read`](crate::Reg::read) this register and get [`bchpbr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHPBR4)

For information about available fields see [`mod@bchpbr4`] module*/
pub type BCHPBR4 = crate::Reg<bchpbr4::BCHPBR4rs>;
///FMC BCH parity bits register 4
pub mod bchpbr4;
/**BCHDSR0 (r) register accessor: FMC BCH decoder status register 0

You can [`read`](crate::Reg::read) this register and get [`bchdsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHDSR0)

For information about available fields see [`mod@bchdsr0`] module*/
pub type BCHDSR0 = crate::Reg<bchdsr0::BCHDSR0rs>;
///FMC BCH decoder status register 0
pub mod bchdsr0;
/**BCHDSR1 (r) register accessor: FMC BCH decoder status register for memory region 1

You can [`read`](crate::Reg::read) this register and get [`bchdsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHDSR1)

For information about available fields see [`mod@bchdsr1`] module*/
pub type BCHDSR1 = crate::Reg<bchdsr1::BCHDSR1rs>;
///FMC BCH decoder status register for memory region 1
pub mod bchdsr1;
/**BCHDSR2 (r) register accessor: FMC BCH decoder status register for memory region 2

You can [`read`](crate::Reg::read) this register and get [`bchdsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHDSR2)

For information about available fields see [`mod@bchdsr2`] module*/
pub type BCHDSR2 = crate::Reg<bchdsr2::BCHDSR2rs>;
///FMC BCH decoder status register for memory region 2
pub mod bchdsr2;
/**BCHDSR3 (r) register accessor: FMC BCH decoder status register for memory region 3

You can [`read`](crate::Reg::read) this register and get [`bchdsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHDSR3)

For information about available fields see [`mod@bchdsr3`] module*/
pub type BCHDSR3 = crate::Reg<bchdsr3::BCHDSR3rs>;
///FMC BCH decoder status register for memory region 3
pub mod bchdsr3;
/**BCHDSR4 (r) register accessor: FMC BCH decoder status register for memory region 4

You can [`read`](crate::Reg::read) this register and get [`bchdsr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:BCHDSR4)

For information about available fields see [`mod@bchdsr4`] module*/
pub type BCHDSR4 = crate::Reg<bchdsr4::BCHDSR4rs>;
///FMC BCH decoder status register for memory region 4
pub mod bchdsr4;
