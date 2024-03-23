#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp1_csr: OPAMP1_CSR,
    opamp2_csr: OPAMP2_CSR,
    opamp3_csr: OPAMP3_CSR,
    _reserved3: [u8; 0x08],
    opamp6_csr: OPAMP6_CSR,
    opamp1_tcmr: OPAMP1_TCMR,
    opamp2_tcmr: OPAMP2_TCMR,
    opamp3_tcmr: OPAMP3_TCMR,
    _reserved7: [u8; 0x08],
    opamp6_tcmr: OPAMP6_TCMR,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    #[doc = "0x04 - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &OPAMP2_CSR {
        &self.opamp2_csr
    }
    #[doc = "0x08 - OPAMP3 control/status register"]
    #[inline(always)]
    pub const fn opamp3_csr(&self) -> &OPAMP3_CSR {
        &self.opamp3_csr
    }
    #[doc = "0x14 - OPAMP6 control/status register"]
    #[inline(always)]
    pub const fn opamp6_csr(&self) -> &OPAMP6_CSR {
        &self.opamp6_csr
    }
    #[doc = "0x18 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_tcmr(&self) -> &OPAMP1_TCMR {
        &self.opamp1_tcmr
    }
    #[doc = "0x1c - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_tcmr(&self) -> &OPAMP2_TCMR {
        &self.opamp2_tcmr
    }
    #[doc = "0x20 - OPAMP3 control/status register"]
    #[inline(always)]
    pub const fn opamp3_tcmr(&self) -> &OPAMP3_TCMR {
        &self.opamp3_tcmr
    }
    #[doc = "0x2c - OPAMP6 control/status register"]
    #[inline(always)]
    pub const fn opamp6_tcmr(&self) -> &OPAMP6_TCMR {
        &self.opamp6_tcmr
    }
}
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`]
module"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP2_CSR (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`]
module"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSRrs>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: OPAMP3 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp3_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp3_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp3_csr`]
module"]
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSRrs>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_csr;
#[doc = "OPAMP1_TCMR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_tcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_tcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_tcmr`]
module"]
pub type OPAMP1_TCMR = crate::Reg<opamp1_tcmr::OPAMP1_TCMRrs>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_tcmr;
#[doc = "OPAMP2_TCMR (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_tcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_tcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_tcmr`]
module"]
pub type OPAMP2_TCMR = crate::Reg<opamp2_tcmr::OPAMP2_TCMRrs>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_tcmr;
#[doc = "OPAMP3_TCMR (rw) register accessor: OPAMP3 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp3_tcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp3_tcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp3_tcmr`]
module"]
pub type OPAMP3_TCMR = crate::Reg<opamp3_tcmr::OPAMP3_TCMRrs>;
#[doc = "OPAMP3 control/status register"]
pub mod opamp3_tcmr;
#[doc = "OPAMP6_CSR (rw) register accessor: OPAMP6 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp6_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp6_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp6_csr`]
module"]
pub type OPAMP6_CSR = crate::Reg<opamp6_csr::OPAMP6_CSRrs>;
#[doc = "OPAMP6 control/status register"]
pub mod opamp6_csr;
#[doc = "OPAMP6_TCMR (rw) register accessor: OPAMP6 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp6_tcmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp6_tcmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp6_tcmr`]
module"]
pub type OPAMP6_TCMR = crate::Reg<opamp6_tcmr::OPAMP6_TCMRrs>;
#[doc = "OPAMP6 control/status register"]
pub mod opamp6_tcmr;
