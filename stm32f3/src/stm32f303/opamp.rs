#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x38],
    opamp1_csr: OPAMP1_CSR,
    opamp2_csr: OPAMP2_CSR,
    opamp3_csr: OPAMP3_CSR,
    opamp4_csr: OPAMP4_CSR,
}
impl RegisterBlock {
    #[doc = "0x38 - OPAMP1 control register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    #[doc = "0x3c - OPAMP2 control register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
    #[doc = "0x40 - OPAMP3 control register"]
    #[inline(always)]
    pub const fn opamp3_csr(&self) -> &OPAMP3_CSR {
        &self.opamp3_csr
    }
    #[doc = "0x44 - OPAMP4 control register"]
    #[inline(always)]
    pub const fn opamp4_csr(&self) -> &OPAMP4_CSR {
        &self.opamp4_csr
    }
}
#[doc = "OPAMP2_CSR (rw) register accessor: OPAMP2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`]
module"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
#[doc = "OPAMP2 control register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: OPAMP3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp3_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp3_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp3_csr`]
module"]
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSRrs>;
#[doc = "OPAMP3 control register"]
pub mod opamp3_csr;
#[doc = "OPAMP4_CSR (rw) register accessor: OPAMP4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp4_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp4_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp4_csr`]
module"]
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSRrs>;
#[doc = "OPAMP4 control register"]
pub mod opamp4_csr;
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`]
module"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
#[doc = "OPAMP1 control register"]
pub mod opamp1_csr;
