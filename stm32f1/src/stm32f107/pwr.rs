#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    csr: CSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register (PWR_CR)"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Power control register (PWR_CR)"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
}
#[doc = "CR (rw) register accessor: Power control register (PWR_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Power control register (PWR_CR)"]
pub mod cr;
#[doc = "CSR (rw) register accessor: Power control register (PWR_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "Power control register (PWR_CR)"]
pub mod csr;
