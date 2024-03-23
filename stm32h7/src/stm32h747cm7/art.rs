#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctr: CTR,
}
impl RegisterBlock {
    #[doc = "0x00 - control register"]
    #[inline(always)]
    pub const fn ctr(&self) -> &CTR {
        &self.ctr
    }
}
#[doc = "CTR (rw) register accessor: control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
pub type CTR = crate::Reg<ctr::CTRrs>;
#[doc = "control register"]
pub mod ctr;
