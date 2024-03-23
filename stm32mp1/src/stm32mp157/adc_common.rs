#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: CSR,
    _reserved1: [u8; 0x04],
    ccr: CCR,
    cdr: CDR,
    cdr2: CDR2,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Common status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x08 - ADC common control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x0c - Common regular data register for dual mode"]
    #[inline(always)]
    pub const fn cdr(&self) -> &CDR {
        &self.cdr
    }
    #[doc = "0x10 - Common regular data register for dual mode"]
    #[inline(always)]
    pub const fn cdr2(&self) -> &CDR2 {
        &self.cdr2
    }
}
#[doc = "CSR (r) register accessor: ADC Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: ADC common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "ADC common control register"]
pub mod ccr;
#[doc = "CDR (r) register accessor: Common regular data register for dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr`]
module"]
pub type CDR = crate::Reg<cdr::CDRrs>;
#[doc = "Common regular data register for dual mode"]
pub mod cdr;
#[doc = "CDR2 (r) register accessor: Common regular data register for dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdr2`]
module"]
pub type CDR2 = crate::Reg<cdr2::CDR2rs>;
#[doc = "Common regular data register for dual mode"]
pub mod cdr2;
