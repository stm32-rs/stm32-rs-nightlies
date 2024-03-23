#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    _reserved1: [u8; 0x0c],
    lckvtr1: LCKVTR1,
    lckvtr2: LCKVTR2,
    _reserved3: [u8; 0xe8],
    vctr: [VCTR; 64],
}
impl RegisterBlock {
    #[doc = "0x00 - MPCBB control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x10 - MPCBB control register"]
    #[inline(always)]
    pub const fn lckvtr1(&self) -> &LCKVTR1 {
        &self.lckvtr1
    }
    #[doc = "0x14 - MPCBB control register"]
    #[inline(always)]
    pub const fn lckvtr2(&self) -> &LCKVTR2 {
        &self.lckvtr2
    }
    #[doc = "0x100..0x200 - MPCBBx vector register"]
    #[inline(always)]
    pub const fn vctr(&self, n: usize) -> &VCTR {
        &self.vctr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x200 - MPCBBx vector register"]
    #[inline(always)]
    pub fn vctr_iter(&self) -> impl Iterator<Item = &VCTR> {
        self.vctr.iter()
    }
}
#[doc = "CR (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "MPCBB control register"]
pub mod cr;
#[doc = "LCKVTR1 (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckvtr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckvtr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckvtr1`]
module"]
pub type LCKVTR1 = crate::Reg<lckvtr1::LCKVTR1rs>;
#[doc = "MPCBB control register"]
pub mod lckvtr1;
#[doc = "LCKVTR2 (rw) register accessor: MPCBB control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lckvtr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lckvtr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lckvtr2`]
module"]
pub type LCKVTR2 = crate::Reg<lckvtr2::LCKVTR2rs>;
#[doc = "MPCBB control register"]
pub mod lckvtr2;
#[doc = "VCTR (rw) register accessor: MPCBBx vector register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vctr`]
module"]
pub type VCTR = crate::Reg<vctr::VCTRrs>;
#[doc = "MPCBBx vector register"]
pub mod vctr;
