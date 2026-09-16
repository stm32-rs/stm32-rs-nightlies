#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bcr1: BCR1,
    btr: (),
    _reserved2: [u8; 0x04],
    bcr: (),
    _reserved3: [u8; 0x78],
    pcr: PCR,
    sr: SR,
    pmem: PMEM,
    patt: PATT,
    _reserved7: [u8; 0x04],
    eccr: ECCR,
    _reserved8: [u8; 0x6c],
    bwtr: (),
    _reserved9: [u8; 0x3c],
    sdcr1: SDCR1,
    sdcr2: SDCR2,
    sdtr: [SDTR; 2],
    sdcmr: SDCMR,
    sdrtr: SDRTR,
    sdsr: SDSR,
}
impl RegisterBlock {
    ///0x00 - SRAM/NOR-Flash chip-select control register 1
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    ///0x04..0x14 - SRAM/NOR-Flash chip-select timing register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BTR1` register.</div>
    #[inline(always)]
    pub const fn btr(&self, n: usize) -> &BTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x04..0x14 - SRAM/NOR-Flash chip-select timing register %s
    #[inline(always)]
    pub fn btr_iter(&self) -> impl Iterator<Item = &BTR> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        })
    }
    ///0x04 - SRAM/NOR-Flash chip-select timing register 1
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR {
        self.btr(0)
    }
    ///0x0c - SRAM/NOR-Flash chip-select timing register 2
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR {
        self.btr(1)
    }
    ///0x14 - SRAM/NOR-Flash chip-select timing register 3
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR {
        self.btr(2)
    }
    ///0x1c - SRAM/NOR-Flash chip-select timing register 4
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR {
        self.btr(3)
    }
    ///0x08..0x14 - SRAM/NOR-Flash chip-select control register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BCR2` register.</div>
    #[inline(always)]
    pub const fn bcr(&self, n: usize) -> &BCR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x08..0x14 - SRAM/NOR-Flash chip-select control register %s
    #[inline(always)]
    pub fn bcr_iter(&self) -> impl Iterator<Item = &BCR> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(8 * n)
                .cast()
        })
    }
    ///0x08 - SRAM/NOR-Flash chip-select control register 2
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR {
        self.bcr(0)
    }
    ///0x10 - SRAM/NOR-Flash chip-select control register 3
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR {
        self.bcr(1)
    }
    ///0x18 - SRAM/NOR-Flash chip-select control register 4
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR {
        self.bcr(2)
    }
    ///0x80 - PC Card/NAND Flash control register
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
    ///0x94 - ECC result register
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    ///0x104..0x114 - SRAM/NOR-Flash write timing registers %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BWTR1` register.</div>
    #[inline(always)]
    pub const fn bwtr(&self, n: usize) -> &BWTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        }
    }
    ///Iterator for array of:
    ///0x104..0x114 - SRAM/NOR-Flash write timing registers %s
    #[inline(always)]
    pub fn bwtr_iter(&self) -> impl Iterator<Item = &BWTR> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        })
    }
    ///0x104 - SRAM/NOR-Flash write timing registers 1
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR {
        self.bwtr(0)
    }
    ///0x10c - SRAM/NOR-Flash write timing registers 2
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR {
        self.bwtr(1)
    }
    ///0x114 - SRAM/NOR-Flash write timing registers 3
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR {
        self.bwtr(2)
    }
    ///0x11c - SRAM/NOR-Flash write timing registers 4
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR {
        self.bwtr(3)
    }
    ///0x140 - SDRAM Control Register 1
    #[inline(always)]
    pub const fn sdcr1(&self) -> &SDCR1 {
        &self.sdcr1
    }
    ///0x144 - SDRAM Control Register 2
    #[inline(always)]
    pub const fn sdcr2(&self) -> &SDCR2 {
        &self.sdcr2
    }
    ///0x148..0x150 - SDRAM Timing register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `SDTR1` register.</div>
    #[inline(always)]
    pub const fn sdtr(&self, n: usize) -> &SDTR {
        &self.sdtr[n]
    }
    ///Iterator for array of:
    ///0x148..0x150 - SDRAM Timing register %s
    #[inline(always)]
    pub fn sdtr_iter(&self) -> impl Iterator<Item = &SDTR> {
        self.sdtr.iter()
    }
    ///0x148 - SDRAM Timing register 1
    #[inline(always)]
    pub const fn sdtr1(&self) -> &SDTR {
        self.sdtr(0)
    }
    ///0x14c - SDRAM Timing register 2
    #[inline(always)]
    pub const fn sdtr2(&self) -> &SDTR {
        self.sdtr(1)
    }
    ///0x150 - SDRAM Command Mode register
    #[inline(always)]
    pub const fn sdcmr(&self) -> &SDCMR {
        &self.sdcmr
    }
    ///0x154 - SDRAM Refresh Timer register
    #[inline(always)]
    pub const fn sdrtr(&self) -> &SDRTR {
        &self.sdrtr
    }
    ///0x158 - SDRAM Status register
    #[inline(always)]
    pub const fn sdsr(&self) -> &SDSR {
        &self.sdsr
    }
}
/**BCR1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1

You can [`read`](crate::Reg::read) this register and get [`bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:BCR1)

For information about available fields see [`mod@bcr1`] module*/
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
///SRAM/NOR-Flash chip-select control register 1
pub mod bcr1;
/**BTR (rw) register accessor: SRAM/NOR-Flash chip-select timing register %s

You can [`read`](crate::Reg::read) this register and get [`btr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:BTR[1])

For information about available fields see [`mod@btr`] module*/
pub type BTR = crate::Reg<btr::BTRrs>;
///SRAM/NOR-Flash chip-select timing register %s
pub mod btr;
/**BCR (rw) register accessor: SRAM/NOR-Flash chip-select control register %s

You can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:BCR[2])

For information about available fields see [`mod@bcr`] module*/
pub type BCR = crate::Reg<bcr::BCRrs>;
///SRAM/NOR-Flash chip-select control register %s
pub mod bcr;
/**PCR (rw) register accessor: PC Card/NAND Flash control register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:PCR)

For information about available fields see [`mod@pcr`] module*/
pub type PCR = crate::Reg<pcr::PCRrs>;
///PC Card/NAND Flash control register
pub mod pcr;
/**SR (rw) register accessor: FIFO status and interrupt register

You can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SR)

For information about available fields see [`mod@sr`] module*/
pub type SR = crate::Reg<sr::SRrs>;
///FIFO status and interrupt register
pub mod sr;
/**PMEM (rw) register accessor: Common memory space timing register

You can [`read`](crate::Reg::read) this register and get [`pmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:PMEM)

For information about available fields see [`mod@pmem`] module*/
pub type PMEM = crate::Reg<pmem::PMEMrs>;
///Common memory space timing register
pub mod pmem;
/**PATT (rw) register accessor: Attribute memory space timing register

You can [`read`](crate::Reg::read) this register and get [`patt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:PATT)

For information about available fields see [`mod@patt`] module*/
pub type PATT = crate::Reg<patt::PATTrs>;
///Attribute memory space timing register
pub mod patt;
/**ECCR (r) register accessor: ECC result register

You can [`read`](crate::Reg::read) this register and get [`eccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:ECCR)

For information about available fields see [`mod@eccr`] module*/
pub type ECCR = crate::Reg<eccr::ECCRrs>;
///ECC result register
pub mod eccr;
/**BWTR (rw) register accessor: SRAM/NOR-Flash write timing registers %s

You can [`read`](crate::Reg::read) this register and get [`bwtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:BWTR[1])

For information about available fields see [`mod@bwtr`] module*/
pub type BWTR = crate::Reg<bwtr::BWTRrs>;
///SRAM/NOR-Flash write timing registers %s
pub mod bwtr;
/**SDCR1 (rw) register accessor: SDRAM Control Register 1

You can [`read`](crate::Reg::read) this register and get [`sdcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SDCR1)

For information about available fields see [`mod@sdcr1`] module*/
pub type SDCR1 = crate::Reg<sdcr1::SDCR1rs>;
///SDRAM Control Register 1
pub mod sdcr1;
/**SDCR2 (rw) register accessor: SDRAM Control Register 2

You can [`read`](crate::Reg::read) this register and get [`sdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SDCR2)

For information about available fields see [`mod@sdcr2`] module*/
pub type SDCR2 = crate::Reg<sdcr2::SDCR2rs>;
///SDRAM Control Register 2
pub mod sdcr2;
/**SDTR (rw) register accessor: SDRAM Timing register %s

You can [`read`](crate::Reg::read) this register and get [`sdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SDTR[1])

For information about available fields see [`mod@sdtr`] module*/
pub type SDTR = crate::Reg<sdtr::SDTRrs>;
///SDRAM Timing register %s
pub mod sdtr;
/**SDCMR (rw) register accessor: SDRAM Command Mode register

You can [`read`](crate::Reg::read) this register and get [`sdcmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SDCMR)

For information about available fields see [`mod@sdcmr`] module*/
pub type SDCMR = crate::Reg<sdcmr::SDCMRrs>;
///SDRAM Command Mode register
pub mod sdcmr;
/**SDRTR (rw) register accessor: SDRAM Refresh Timer register

You can [`read`](crate::Reg::read) this register and get [`sdrtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdrtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SDRTR)

For information about available fields see [`mod@sdrtr`] module*/
pub type SDRTR = crate::Reg<sdrtr::SDRTRrs>;
///SDRAM Refresh Timer register
pub mod sdrtr;
/**SDSR (r) register accessor: SDRAM Status register

You can [`read`](crate::Reg::read) this register and get [`sdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F745.html#FMC:SDSR)

For information about available fields see [`mod@sdsr`] module*/
pub type SDSR = crate::Reg<sdsr::SDSRrs>;
///SDRAM Status register
pub mod sdsr;
