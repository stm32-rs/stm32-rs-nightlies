#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    wrfr: WRFR,
    cwrfr: CWRFR,
    rdfr: RDFR,
    crdfr: CRDFR,
    sr: SR,
    clrfr: CLRFR,
    dinr: [DINR; 32],
    doutr: [DOUTR; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - MDIOS write flag register"]
    #[inline(always)]
    pub const fn wrfr(&self) -> &WRFR {
        &self.wrfr
    }
    #[doc = "0x08 - MDIOS clear write flag register"]
    #[inline(always)]
    pub const fn cwrfr(&self) -> &CWRFR {
        &self.cwrfr
    }
    #[doc = "0x0c - MDIOS read flag register"]
    #[inline(always)]
    pub const fn rdfr(&self) -> &RDFR {
        &self.rdfr
    }
    #[doc = "0x10 - MDIOS clear read flag register"]
    #[inline(always)]
    pub const fn crdfr(&self) -> &CRDFR {
        &self.crdfr
    }
    #[doc = "0x14 - MDIOS status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x18 - MDIOS clear flag register"]
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    #[doc = "0x1c..0x9c - MDIOS input data register %s"]
    #[inline(always)]
    pub const fn dinr(&self, n: usize) -> &DINR {
        &self.dinr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x9c - MDIOS input data register %s"]
    #[inline(always)]
    pub fn dinr_iter(&self) -> impl Iterator<Item = &DINR> {
        self.dinr.iter()
    }
    #[doc = "0x9c..0x11c - MDIOS output data register %s"]
    #[inline(always)]
    pub const fn doutr(&self, n: usize) -> &DOUTR {
        &self.doutr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0x11c - MDIOS output data register %s"]
    #[inline(always)]
    pub fn doutr_iter(&self) -> impl Iterator<Item = &DOUTR> {
        self.doutr.iter()
    }
}
#[doc = "CR (rw) register accessor: MDIOS configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "MDIOS configuration register"]
pub mod cr;
#[doc = "WRFR (r) register accessor: MDIOS write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrfr`]
module"]
pub type WRFR = crate::Reg<wrfr::WRFRrs>;
#[doc = "MDIOS write flag register"]
pub mod wrfr;
#[doc = "CWRFR (rw) register accessor: MDIOS clear write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwrfr`]
module"]
pub type CWRFR = crate::Reg<cwrfr::CWRFRrs>;
#[doc = "MDIOS clear write flag register"]
pub mod cwrfr;
#[doc = "RDFR (r) register accessor: MDIOS read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdfr`]
module"]
pub type RDFR = crate::Reg<rdfr::RDFRrs>;
#[doc = "MDIOS read flag register"]
pub mod rdfr;
#[doc = "CRDFR (rw) register accessor: MDIOS clear read flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crdfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crdfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crdfr`]
module"]
pub type CRDFR = crate::Reg<crdfr::CRDFRrs>;
#[doc = "MDIOS clear read flag register"]
pub mod crdfr;
#[doc = "SR (r) register accessor: MDIOS status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "MDIOS status register"]
pub mod sr;
#[doc = "CLRFR (rw) register accessor: MDIOS clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrfr`]
module"]
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
#[doc = "MDIOS clear flag register"]
pub mod clrfr;
#[doc = "DINR (r) register accessor: MDIOS input data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dinr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dinr`]
module"]
pub type DINR = crate::Reg<dinr::DINRrs>;
#[doc = "MDIOS input data register %s"]
pub mod dinr;
#[doc = "DOUTR (rw) register accessor: MDIOS output data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutr`]
module"]
pub type DOUTR = crate::Reg<doutr::DOUTRrs>;
#[doc = "MDIOS output data register %s"]
pub mod doutr;
