#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: DR,
    idr: IDR,
    cr: CR,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x04 - Independent Data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &IDR {
        &self.idr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
}
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "Data register"]
pub mod dr;
#[doc = "IDR (rw) register accessor: Independent Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
pub type IDR = crate::Reg<idr::IDRrs>;
#[doc = "Independent Data register"]
pub mod idr;
#[doc = "CR (w) register accessor: Control register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control register"]
pub mod cr;
