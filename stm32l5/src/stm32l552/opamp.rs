#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp1_csr: OPAMP1_CSR,
    opamp1_otr: OPAMP1_OTR,
    opamp1_lpotr: OPAMP1_LPOTR,
    _reserved3: [u8; 0x04],
    opamp2_crs: OPAMP2_CRS,
    opamp2_otr: OPAMP2_OTR,
    opamp2_lpotr: OPAMP2_LPOTR,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &OPAMP1_CSR {
        &self.opamp1_csr
    }
    #[doc = "0x04 - OPAMP1 offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp1_otr(&self) -> &OPAMP1_OTR {
        &self.opamp1_otr
    }
    #[doc = "0x08 - OPAMP1 offset trimming register in low-powe mode"]
    #[inline(always)]
    pub const fn opamp1_lpotr(&self) -> &OPAMP1_LPOTR {
        &self.opamp1_lpotr
    }
    #[doc = "0x10 - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_crs(&self) -> &OPAMP2_CRS {
        &self.opamp2_crs
    }
    #[doc = "0x14 - OPAMP2 offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp2_otr(&self) -> &OPAMP2_OTR {
        &self.opamp2_otr
    }
    #[doc = "0x18 - OPAMP2 offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn opamp2_lpotr(&self) -> &OPAMP2_LPOTR {
        &self.opamp2_lpotr
    }
}
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`]
module"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSRrs>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1_OTR (rw) register accessor: OPAMP1 offset trimming register in normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_otr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_otr`]
module"]
pub type OPAMP1_OTR = crate::Reg<opamp1_otr::OPAMP1_OTRrs>;
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1_LPOTR (rw) register accessor: OPAMP1 offset trimming register in low-powe mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp1_lpotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp1_lpotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_lpotr`]
module"]
pub type OPAMP1_LPOTR = crate::Reg<opamp1_lpotr::OPAMP1_LPOTRrs>;
#[doc = "OPAMP1 offset trimming register in low-powe mode"]
pub mod opamp1_lpotr;
#[doc = "OPAMP2_CRS (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_crs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_crs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_crs`]
module"]
pub type OPAMP2_CRS = crate::Reg<opamp2_crs::OPAMP2_CRSrs>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_crs;
#[doc = "OPAMP2_OTR (rw) register accessor: OPAMP2 offset trimming register in normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_otr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_otr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_otr`]
module"]
pub type OPAMP2_OTR = crate::Reg<opamp2_otr::OPAMP2_OTRrs>;
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub mod opamp2_otr;
#[doc = "OPAMP2_LPOTR (rw) register accessor: OPAMP2 offset trimming register in low-power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opamp2_lpotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opamp2_lpotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_lpotr`]
module"]
pub type OPAMP2_LPOTR = crate::Reg<opamp2_lpotr::OPAMP2_LPOTRrs>;
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub mod opamp2_lpotr;
