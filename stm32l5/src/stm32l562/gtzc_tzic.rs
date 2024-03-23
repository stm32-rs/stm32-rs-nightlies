#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier1: IER1,
    ier2: IER2,
    ier3: IER3,
    _reserved3: [u8; 0x04],
    sr1: SR1,
    sr2: SR2,
    sr3: SR3,
    _reserved6: [u8; 0x04],
    fcr1: FCR1,
    fcr2: FCR2,
    fcr3: FCR3,
}
impl RegisterBlock {
    #[doc = "0x00 - TZIC interrupt enable register 1"]
    #[inline(always)]
    pub const fn ier1(&self) -> &IER1 {
        &self.ier1
    }
    #[doc = "0x04 - TZIC interrupt enable register 2"]
    #[inline(always)]
    pub const fn ier2(&self) -> &IER2 {
        &self.ier2
    }
    #[doc = "0x08 - TZIC interrupt enable register 3"]
    #[inline(always)]
    pub const fn ier3(&self) -> &IER3 {
        &self.ier3
    }
    #[doc = "0x10 - TZIC interrupt status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x14 - TZIC interrupt status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x18 - TZIC interrupt status register 3"]
    #[inline(always)]
    pub const fn sr3(&self) -> &SR3 {
        &self.sr3
    }
    #[doc = "0x20 - TZIC interrupt clear register 1"]
    #[inline(always)]
    pub const fn fcr1(&self) -> &FCR1 {
        &self.fcr1
    }
    #[doc = "0x24 - TZIC interrupt clear register 2"]
    #[inline(always)]
    pub const fn fcr2(&self) -> &FCR2 {
        &self.fcr2
    }
    #[doc = "0x28 - TZIC interrupt clear register 3"]
    #[inline(always)]
    pub const fn fcr3(&self) -> &FCR3 {
        &self.fcr3
    }
}
#[doc = "IER1 (rw) register accessor: TZIC interrupt enable register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier1`]
module"]
pub type IER1 = crate::Reg<ier1::IER1rs>;
#[doc = "TZIC interrupt enable register 1"]
pub mod ier1;
#[doc = "IER2 (rw) register accessor: TZIC interrupt enable register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier2`]
module"]
pub type IER2 = crate::Reg<ier2::IER2rs>;
#[doc = "TZIC interrupt enable register 2"]
pub mod ier2;
#[doc = "IER3 (rw) register accessor: TZIC interrupt enable register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier3`]
module"]
pub type IER3 = crate::Reg<ier3::IER3rs>;
#[doc = "TZIC interrupt enable register 3"]
pub mod ier3;
#[doc = "SR1 (r) register accessor: TZIC interrupt status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1rs>;
#[doc = "TZIC interrupt status register 1"]
pub mod sr1;
#[doc = "SR2 (rw) register accessor: TZIC interrupt status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2rs>;
#[doc = "TZIC interrupt status register 2"]
pub mod sr2;
#[doc = "SR3 (rw) register accessor: TZIC interrupt status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr3`]
module"]
pub type SR3 = crate::Reg<sr3::SR3rs>;
#[doc = "TZIC interrupt status register 3"]
pub mod sr3;
#[doc = "FCR1 (w) register accessor: TZIC interrupt clear register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr1`]
module"]
pub type FCR1 = crate::Reg<fcr1::FCR1rs>;
#[doc = "TZIC interrupt clear register 1"]
pub mod fcr1;
#[doc = "FCR2 (rw) register accessor: TZIC interrupt clear register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr2`]
module"]
pub type FCR2 = crate::Reg<fcr2::FCR2rs>;
#[doc = "TZIC interrupt clear register 2"]
pub mod fcr2;
#[doc = "FCR3 (rw) register accessor: TZIC interrupt clear register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr3`]
module"]
pub type FCR3 = crate::Reg<fcr3::FCR3rs>;
#[doc = "TZIC interrupt clear register 3"]
pub mod fcr3;
