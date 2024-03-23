#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ccr: CCR,
}
impl RegisterBlock {
    #[doc = "0x04 - ADC common control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
}
#[doc = "CCR (rw) register accessor: ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "ADC common control register"]
pub mod ccr;
