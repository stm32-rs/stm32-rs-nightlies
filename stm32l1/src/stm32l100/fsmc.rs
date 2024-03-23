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
    #[doc = "0x00 - BCR1"]
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    #[doc = "0x04..0x14 - BTR%s"]
    #[inline(always)]
    pub const fn btr(&self, n: usize) -> &BTR {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - BTR%s"]
    #[inline(always)]
    pub fn btr_iter(&self) -> impl Iterator<Item = &BTR> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(8 * n).cast() })
    }
    #[doc = "0x04 - BTR1"]
    #[inline(always)]
    pub const fn btr1(&self) -> &BTR {
        self.btr(0)
    }
    #[doc = "0x0c - BTR2"]
    #[inline(always)]
    pub const fn btr2(&self) -> &BTR {
        self.btr(1)
    }
    #[doc = "0x14 - BTR3"]
    #[inline(always)]
    pub const fn btr3(&self) -> &BTR {
        self.btr(2)
    }
    #[doc = "0x1c - BTR4"]
    #[inline(always)]
    pub const fn btr4(&self) -> &BTR {
        self.btr(3)
    }
    #[doc = "0x08..0x14 - BCR%s"]
    #[inline(always)]
    pub const fn bcr(&self, n: usize) -> &BCR {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x14 - BCR%s"]
    #[inline(always)]
    pub fn bcr_iter(&self) -> impl Iterator<Item = &BCR> {
        (0..3)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(8 * n).cast() })
    }
    #[doc = "0x08 - BCR2"]
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR {
        self.bcr(0)
    }
    #[doc = "0x10 - BCR3"]
    #[inline(always)]
    pub const fn bcr3(&self) -> &BCR {
        self.bcr(1)
    }
    #[doc = "0x18 - BCR4"]
    #[inline(always)]
    pub const fn bcr4(&self) -> &BCR {
        self.bcr(2)
    }
    #[doc = "0x104..0x114 - BWTR%s"]
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
    #[doc = "0x104..0x114 - BWTR%s"]
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
    #[doc = "0x104 - BWTR1"]
    #[inline(always)]
    pub const fn bwtr1(&self) -> &BWTR {
        self.bwtr(0)
    }
    #[doc = "0x10c - BWTR2"]
    #[inline(always)]
    pub const fn bwtr2(&self) -> &BWTR {
        self.bwtr(1)
    }
    #[doc = "0x114 - BWTR3"]
    #[inline(always)]
    pub const fn bwtr3(&self) -> &BWTR {
        self.bwtr(2)
    }
    #[doc = "0x11c - BWTR4"]
    #[inline(always)]
    pub const fn bwtr4(&self) -> &BWTR {
        self.bwtr(3)
    }
}
pub use bcr as bcr1;
pub use BCR as BCR1;
#[doc = "BTR (rw) register accessor: BTR%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btr`]
module"]
pub type BTR = crate::Reg<btr::BTRrs>;
#[doc = "BTR%s"]
pub mod btr;
#[doc = "BCR (rw) register accessor: BCR%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`]
module"]
pub type BCR = crate::Reg<bcr::BCRrs>;
#[doc = "BCR%s"]
pub mod bcr;
#[doc = "BWTR (rw) register accessor: BWTR%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bwtr`]
module"]
pub type BWTR = crate::Reg<bwtr::BWTRrs>;
#[doc = "BWTR%s"]
pub mod bwtr;
