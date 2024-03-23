#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: CSR,
    otr: OTR,
    lpotr: LPOTR,
}
impl RegisterBlock {
    #[doc = "0x00 - control/status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x04 - offset trimming register for normal mode"]
    #[inline(always)]
    pub const fn otr(&self) -> &OTR {
        &self.otr
    }
    #[doc = "0x08 - OPAMP offset trimming register for low power mode"]
    #[inline(always)]
    pub const fn lpotr(&self) -> &LPOTR {
        &self.lpotr
    }
}
#[doc = "CSR (rw) register accessor: control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSRrs>;
#[doc = "control/status register"]
pub mod csr;
#[doc = "OTR (rw) register accessor: offset trimming register for normal mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otr`]
module"]
pub type OTR = crate::Reg<otr::OTRrs>;
#[doc = "offset trimming register for normal mode"]
pub mod otr;
#[doc = "LPOTR (rw) register accessor: OPAMP offset trimming register for low power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpotr`]
module"]
pub type LPOTR = crate::Reg<lpotr::LPOTRrs>;
#[doc = "OPAMP offset trimming register for low power mode"]
pub mod lpotr;
