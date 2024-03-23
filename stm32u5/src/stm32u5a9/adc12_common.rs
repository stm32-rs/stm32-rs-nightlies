#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc12_csr: ADC12_CSR,
    _reserved1: [u8; 0x04],
    adc12_ccr: ADC12_CCR,
    adc12_cdr: ADC12_CDR,
    adc12_cdr2: ADC12_CDR2,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC common status register"]
    #[inline(always)]
    pub const fn adc12_csr(&self) -> &ADC12_CSR {
        &self.adc12_csr
    }
    #[doc = "0x08 - ADC_CCR system control register"]
    #[inline(always)]
    pub const fn adc12_ccr(&self) -> &ADC12_CCR {
        &self.adc12_ccr
    }
    #[doc = "0x0c - ADC common regular data register for dual mode"]
    #[inline(always)]
    pub const fn adc12_cdr(&self) -> &ADC12_CDR {
        &self.adc12_cdr
    }
    #[doc = "0x10 - ADC common regular data register for 32-bit dual mode"]
    #[inline(always)]
    pub const fn adc12_cdr2(&self) -> &ADC12_CDR2 {
        &self.adc12_cdr2
    }
}
#[doc = "ADC12_CSR (r) register accessor: ADC common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_csr`]
module"]
pub type ADC12_CSR = crate::Reg<adc12_csr::ADC12_CSRrs>;
#[doc = "ADC common status register"]
pub mod adc12_csr;
#[doc = "ADC12_CCR (rw) register accessor: ADC_CCR system control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc12_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_ccr`]
module"]
pub type ADC12_CCR = crate::Reg<adc12_ccr::ADC12_CCRrs>;
#[doc = "ADC_CCR system control register"]
pub mod adc12_ccr;
#[doc = "ADC12_CDR (r) register accessor: ADC common regular data register for dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_cdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_cdr`]
module"]
pub type ADC12_CDR = crate::Reg<adc12_cdr::ADC12_CDRrs>;
#[doc = "ADC common regular data register for dual mode"]
pub mod adc12_cdr;
#[doc = "ADC12_CDR2 (r) register accessor: ADC common regular data register for 32-bit dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc12_cdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc12_cdr2`]
module"]
pub type ADC12_CDR2 = crate::Reg<adc12_cdr2::ADC12_CDR2rs>;
#[doc = "ADC common regular data register for 32-bit dual mode"]
pub mod adc12_cdr2;
