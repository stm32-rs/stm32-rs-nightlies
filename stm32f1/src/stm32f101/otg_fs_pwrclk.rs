#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "PCGCCTL (rw) register accessor: OTG_FS power and clock gating control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcgcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`]
module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTLrs>;
#[doc = "OTG_FS power and clock gating control register"]
pub mod pcgcctl;
