#[repr(C)]
#[doc = "Register block"]
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
    sdcr: [SDCR; 2],
    sdtr: [SDTR; 2],
    sdcmr: SDCMR,
    sdrtr: SDRTR,
    sdsr: SDSR,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    #[doc = "0x04..0x14 - SRAM/NOR-Flash chip-select timing register %s"]
    #[inline(always)]
    pub const fn btr(&self, n: usize) -> &BTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - SRAM/NOR-Flash chip-select timing register %s"]
    #[inline(always)]
    pub fn btr_iter(&self) -> impl Iterator<Item = &BTR> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(8 * n).cast() })
    }
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR {
        self.btr(0)
    }
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR {
        self.btr(1)
    }
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR {
        self.btr(2)
    }
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR {
        self.btr(3)
    }
    #[doc = "0x08..0x14 - SRAM/NOR-Flash chip-select control register %s"]
    #[inline(always)]
    pub const fn bcr(&self, n: usize) -> &BCR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x14 - SRAM/NOR-Flash chip-select control register %s"]
    #[inline(always)]
    pub fn bcr_iter(&self) -> impl Iterator<Item = &BCR> {
        (0..3)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(8 * n).cast() })
    }
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR {
        self.bcr(0)
    }
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR {
        self.bcr(1)
    }
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR {
        self.bcr(2)
    }
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    #[inline(always)]
    pub const fn pcr(&self) -> &PCR {
        &self.pcr
    }
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x88 - Common memory space timing register 3"]
    #[inline(always)]
    pub const fn pmem(&self) -> &PMEM {
        &self.pmem
    }
    #[doc = "0x8c - Attribute memory space timing register 3"]
    #[inline(always)]
    pub const fn patt(&self) -> &PATT {
        &self.patt
    }
    #[doc = "0x94 - ECC result register 3"]
    #[inline(always)]
    pub const fn eccr(&self) -> &ECCR {
        &self.eccr
    }
    #[doc = "0x104..0x114 - SRAM/NOR-Flash write timing registers %s"]
    #[inline(always)]
    pub const fn bwtr(&self, n: usize) -> &BWTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x114 - SRAM/NOR-Flash write timing registers %s"]
    #[inline(always)]
    pub fn bwtr_iter(&self) -> impl Iterator<Item = &BWTR> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR {
        self.bwtr(0)
    }
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR {
        self.bwtr(1)
    }
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR {
        self.bwtr(2)
    }
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR {
        self.bwtr(3)
    }
    #[doc = "0x140..0x148 - SDRAM Control Register %s"]
    #[inline(always)]
    pub const fn sdcr(&self, n: usize) -> &SDCR {
        &self.sdcr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x148 - SDRAM Control Register %s"]
    #[inline(always)]
    pub fn sdcr_iter(&self) -> impl Iterator<Item = &SDCR> {
        self.sdcr.iter()
    }
    #[doc = "0x140 - SDRAM Control Register 1"]
    #[inline(always)]
    pub const fn sdcr1(&self) -> &SDCR {
        self.sdcr(0)
    }
    #[doc = "0x144 - SDRAM Control Register 2"]
    #[inline(always)]
    pub const fn sdcr2(&self) -> &SDCR {
        self.sdcr(1)
    }
    #[doc = "0x148..0x150 - SDRAM Timing register %s"]
    #[inline(always)]
    pub const fn sdtr(&self, n: usize) -> &SDTR {
        &self.sdtr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x148..0x150 - SDRAM Timing register %s"]
    #[inline(always)]
    pub fn sdtr_iter(&self) -> impl Iterator<Item = &SDTR> {
        self.sdtr.iter()
    }
    #[doc = "0x148 - SDRAM Timing register 1"]
    #[inline(always)]
    pub const fn sdtr1(&self) -> &SDTR {
        self.sdtr(0)
    }
    #[doc = "0x14c - SDRAM Timing register 2"]
    #[inline(always)]
    pub const fn sdtr2(&self) -> &SDTR {
        self.sdtr(1)
    }
    #[doc = "0x150 - SDRAM Command Mode register"]
    #[inline(always)]
    pub const fn sdcmr(&self) -> &SDCMR {
        &self.sdcmr
    }
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    #[inline(always)]
    pub const fn sdrtr(&self) -> &SDRTR {
        &self.sdrtr
    }
    #[doc = "0x158 - SDRAM Status register"]
    #[inline(always)]
    pub const fn sdsr(&self) -> &SDSR {
        &self.sdsr
    }
}
#[doc = "BCR1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr1`]
module"]
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "BTR (rw) register accessor: SRAM/NOR-Flash chip-select timing register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr`]
module"]
pub type BTR = crate::Reg<btr::BTRrs>;
#[doc = "SRAM/NOR-Flash chip-select timing register %s"]
pub mod btr;
#[doc = "BCR (rw) register accessor: SRAM/NOR-Flash chip-select control register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCRrs>;
#[doc = "SRAM/NOR-Flash chip-select control register %s"]
pub mod bcr;
#[doc = "PCR (rw) register accessor: PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`]
module"]
pub type PCR = crate::Reg<pcr::PCRrs>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod pcr;
#[doc = "SR (rw) register accessor: FIFO status and interrupt register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "FIFO status and interrupt register 3"]
pub mod sr;
#[doc = "PMEM (rw) register accessor: Common memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmem`]
module"]
pub type PMEM = crate::Reg<pmem::PMEMrs>;
#[doc = "Common memory space timing register 3"]
pub mod pmem;
#[doc = "PATT (rw) register accessor: Attribute memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patt`]
module"]
pub type PATT = crate::Reg<patt::PATTrs>;
#[doc = "Attribute memory space timing register 3"]
pub mod patt;
#[doc = "ECCR (r) register accessor: ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccr`]
module"]
pub type ECCR = crate::Reg<eccr::ECCRrs>;
#[doc = "ECC result register 3"]
pub mod eccr;
#[doc = "BWTR (rw) register accessor: SRAM/NOR-Flash write timing registers %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr`]
module"]
pub type BWTR = crate::Reg<bwtr::BWTRrs>;
#[doc = "SRAM/NOR-Flash write timing registers %s"]
pub mod bwtr;
#[doc = "SDCR (rw) register accessor: SDRAM Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcr`]
module"]
pub type SDCR = crate::Reg<sdcr::SDCRrs>;
#[doc = "SDRAM Control Register %s"]
pub mod sdcr;
#[doc = "SDTR (rw) register accessor: SDRAM Timing register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdtr`]
module"]
pub type SDTR = crate::Reg<sdtr::SDTRrs>;
#[doc = "SDRAM Timing register %s"]
pub mod sdtr;
#[doc = "SDCMR (rw) register accessor: SDRAM Command Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdcmr`]
module"]
pub type SDCMR = crate::Reg<sdcmr::SDCMRrs>;
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRTR (rw) register accessor: SDRAM Refresh Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdrtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdrtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdrtr`]
module"]
pub type SDRTR = crate::Reg<sdrtr::SDRTRrs>;
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDSR (r) register accessor: SDRAM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdsr`]
module"]
pub type SDSR = crate::Reg<sdsr::SDSRrs>;
#[doc = "SDRAM Status register"]
pub mod sdsr;
