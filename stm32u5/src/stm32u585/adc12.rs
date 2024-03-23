#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    adc12_ccr: ADC12_CCR,
}
impl RegisterBlock {
    #[doc = "0x08 - ADC_CCR system control register"]
    #[inline(always)]
    pub const fn adc12_ccr(&self) -> &ADC12_CCR {
        &self.adc12_ccr
    }
}
#[doc = "ADC12_CCR (rw) register accessor: ADC_CCR system control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_ccr`]
module"]
pub type ADC12_CCR = crate::Reg<adc12_ccr::ADC12_CCRrs>;
#[doc = "ADC_CCR system control register"]
pub mod adc12_ccr;
