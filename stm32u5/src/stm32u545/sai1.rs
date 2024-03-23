#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gcr: GCR,
    acr1: ACR1,
    acr2: ACR2,
    afrcr: AFRCR,
    aslotr: ASLOTR,
    aim: AIM,
    asr: ASR,
    aclrfr: ACLRFR,
    adr: ADR,
    bcr1: BCR1,
    bcr2: BCR2,
    bfrcr: BFRCR,
    bslotr: BSLOTR,
    bim: BIM,
    bsr: BSR,
    bclrfr: BCLRFR,
    bdr: BDR,
    pdmcr: PDMCR,
    pdmdly: PDMDLY,
}
impl RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0x04 - A Configuration register 1"]
    #[inline(always)]
    pub const fn acr1(&self) -> &ACR1 {
        &self.acr1
    }
    #[doc = "0x08 - A Configuration register 2"]
    #[inline(always)]
    pub const fn acr2(&self) -> &ACR2 {
        &self.acr2
    }
    #[doc = "0x0c - A frame configuration register"]
    #[inline(always)]
    pub const fn afrcr(&self) -> &AFRCR {
        &self.afrcr
    }
    #[doc = "0x10 - A Slot register"]
    #[inline(always)]
    pub const fn aslotr(&self) -> &ASLOTR {
        &self.aslotr
    }
    #[doc = "0x14 - A Interrupt mask register"]
    #[inline(always)]
    pub const fn aim(&self) -> &AIM {
        &self.aim
    }
    #[doc = "0x18 - A Status register"]
    #[inline(always)]
    pub const fn asr(&self) -> &ASR {
        &self.asr
    }
    #[doc = "0x1c - A Clear flag register"]
    #[inline(always)]
    pub const fn aclrfr(&self) -> &ACLRFR {
        &self.aclrfr
    }
    #[doc = "0x20 - A Data register"]
    #[inline(always)]
    pub const fn adr(&self) -> &ADR {
        &self.adr
    }
    #[doc = "0x24 - B Configuration register 1"]
    #[inline(always)]
    pub const fn bcr1(&self) -> &BCR1 {
        &self.bcr1
    }
    #[doc = "0x28 - B Configuration register 2"]
    #[inline(always)]
    pub const fn bcr2(&self) -> &BCR2 {
        &self.bcr2
    }
    #[doc = "0x2c - B frame configuration register"]
    #[inline(always)]
    pub const fn bfrcr(&self) -> &BFRCR {
        &self.bfrcr
    }
    #[doc = "0x30 - B Slot register"]
    #[inline(always)]
    pub const fn bslotr(&self) -> &BSLOTR {
        &self.bslotr
    }
    #[doc = "0x34 - B Interrupt mask register"]
    #[inline(always)]
    pub const fn bim(&self) -> &BIM {
        &self.bim
    }
    #[doc = "0x38 - B Status register"]
    #[inline(always)]
    pub const fn bsr(&self) -> &BSR {
        &self.bsr
    }
    #[doc = "0x3c - B Clear flag register"]
    #[inline(always)]
    pub const fn bclrfr(&self) -> &BCLRFR {
        &self.bclrfr
    }
    #[doc = "0x40 - B Data register"]
    #[inline(always)]
    pub const fn bdr(&self) -> &BDR {
        &self.bdr
    }
    #[doc = "0x44 - PDM control register"]
    #[inline(always)]
    pub const fn pdmcr(&self) -> &PDMCR {
        &self.pdmcr
    }
    #[doc = "0x48 - PDM delay register"]
    #[inline(always)]
    pub const fn pdmdly(&self) -> &PDMDLY {
        &self.pdmdly
    }
}
#[doc = "GCR (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCRrs>;
#[doc = "Global configuration register"]
pub mod gcr;
#[doc = "ACR1 (rw) register accessor: A Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr1`]
module"]
pub type ACR1 = crate::Reg<acr1::ACR1rs>;
#[doc = "A Configuration register 1"]
pub mod acr1;
#[doc = "BCR1 (rw) register accessor: B Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr1`]
module"]
pub type BCR1 = crate::Reg<bcr1::BCR1rs>;
#[doc = "B Configuration register 1"]
pub mod bcr1;
#[doc = "ACR2 (rw) register accessor: A Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr2`]
module"]
pub type ACR2 = crate::Reg<acr2::ACR2rs>;
#[doc = "A Configuration register 2"]
pub mod acr2;
#[doc = "BCR2 (rw) register accessor: B Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr2`]
module"]
pub type BCR2 = crate::Reg<bcr2::BCR2rs>;
#[doc = "B Configuration register 2"]
pub mod bcr2;
#[doc = "AFRCR (rw) register accessor: A frame configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@afrcr`]
module"]
pub type AFRCR = crate::Reg<afrcr::AFRCRrs>;
#[doc = "A frame configuration register"]
pub mod afrcr;
#[doc = "BFRCR (rw) register accessor: B frame configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bfrcr`]
module"]
pub type BFRCR = crate::Reg<bfrcr::BFRCRrs>;
#[doc = "B frame configuration register"]
pub mod bfrcr;
#[doc = "ASLOTR (rw) register accessor: A Slot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aslotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aslotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aslotr`]
module"]
pub type ASLOTR = crate::Reg<aslotr::ASLOTRrs>;
#[doc = "A Slot register"]
pub mod aslotr;
#[doc = "BSLOTR (rw) register accessor: B Slot register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bslotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bslotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bslotr`]
module"]
pub type BSLOTR = crate::Reg<bslotr::BSLOTRrs>;
#[doc = "B Slot register"]
pub mod bslotr;
#[doc = "AIM (rw) register accessor: A Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aim`]
module"]
pub type AIM = crate::Reg<aim::AIMrs>;
#[doc = "A Interrupt mask register"]
pub mod aim;
#[doc = "BIM (rw) register accessor: B Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bim`]
module"]
pub type BIM = crate::Reg<bim::BIMrs>;
#[doc = "B Interrupt mask register"]
pub mod bim;
#[doc = "ASR (r) register accessor: A Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asr`]
module"]
pub type ASR = crate::Reg<asr::ASRrs>;
#[doc = "A Status register"]
pub mod asr;
#[doc = "BSR (r) register accessor: B Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsr`]
module"]
pub type BSR = crate::Reg<bsr::BSRrs>;
#[doc = "B Status register"]
pub mod bsr;
#[doc = "ACLRFR (w) register accessor: A Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aclrfr`]
module"]
pub type ACLRFR = crate::Reg<aclrfr::ACLRFRrs>;
#[doc = "A Clear flag register"]
pub mod aclrfr;
#[doc = "BCLRFR (w) register accessor: B Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bclrfr`]
module"]
pub type BCLRFR = crate::Reg<bclrfr::BCLRFRrs>;
#[doc = "B Clear flag register"]
pub mod bclrfr;
#[doc = "ADR (rw) register accessor: A Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adr`]
module"]
pub type ADR = crate::Reg<adr::ADRrs>;
#[doc = "A Data register"]
pub mod adr;
#[doc = "BDR (rw) register accessor: B Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdr`]
module"]
pub type BDR = crate::Reg<bdr::BDRrs>;
#[doc = "B Data register"]
pub mod bdr;
#[doc = "PDMCR (rw) register accessor: PDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdmcr`]
module"]
pub type PDMCR = crate::Reg<pdmcr::PDMCRrs>;
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDMDLY (rw) register accessor: PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdmdly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdmdly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdmdly`]
module"]
pub type PDMDLY = crate::Reg<pdmdly::PDMDLYrs>;
#[doc = "PDM delay register"]
pub mod pdmdly;
