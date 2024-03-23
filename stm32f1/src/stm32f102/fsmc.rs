#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bcr1: BCR1,
    btr: (),
    _reserved2: [u8; 0x04],
    bcr: (),
    _reserved3: [u8; 0xfc],
    bwtr: (),
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
#[doc = "BWTR (rw) register accessor: SRAM/NOR-Flash write timing registers %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr`]
module"]
pub type BWTR = crate::Reg<bwtr::BWTRrs>;
#[doc = "SRAM/NOR-Flash write timing registers %s"]
pub mod bwtr;
