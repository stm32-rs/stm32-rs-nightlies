#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcgcr: PCGCR,
}
impl RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    #[inline(always)]
    pub const fn pcgcr(&self) -> &PCGCR {
        &self.pcgcr
    }
}
#[doc = "PCGCR (rw) register accessor: Power and clock gating control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcr`]
module"]
pub type PCGCR = crate::Reg<pcgcr::PCGCRrs>;
#[doc = "Power and clock gating control register"]
pub mod pcgcr;
