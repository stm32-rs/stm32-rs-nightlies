#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sai_gcr: SAI_GCR,
    sai_acr1: SAI_ACR1,
    sai_acr2: SAI_ACR2,
    sai_afrcr: SAI_AFRCR,
    sai_aslotr: SAI_ASLOTR,
    sai_aim: SAI_AIM,
    sai_asr: SAI_ASR,
    sai_aclrfr: SAI_ACLRFR,
    sai_adr: SAI_ADR,
    sai_bcr1: SAI_BCR1,
    sai_bcr2: SAI_BCR2,
    sai_bfrcr: SAI_BFRCR,
    sai_bslotr: SAI_BSLOTR,
    sai_bim: SAI_BIM,
    sai_bsr: SAI_BSR,
    sai_bclrfr: SAI_BCLRFR,
    sai_bdr: SAI_BDR,
    sai_pdmcr: SAI_PDMCR,
    sai_pdmdly: SAI_PDMDLY,
    _reserved19: [u8; 0x03a4],
    sai_hwcfgr: SAI_HWCFGR,
    sai_verr: SAI_VERR,
    sai_ipidr: SAI_IPIDR,
    sai_sidr: SAI_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    #[inline(always)]
    pub const fn sai_gcr(&self) -> &SAI_GCR {
        &self.sai_gcr
    }
    #[doc = "0x04 - Configuration register 1"]
    #[inline(always)]
    pub const fn sai_acr1(&self) -> &SAI_ACR1 {
        &self.sai_acr1
    }
    #[doc = "0x08 - Configuration register 2"]
    #[inline(always)]
    pub const fn sai_acr2(&self) -> &SAI_ACR2 {
        &self.sai_acr2
    }
    #[doc = "0x0c - This register has no meaning in and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn sai_afrcr(&self) -> &SAI_AFRCR {
        &self.sai_afrcr
    }
    #[doc = "0x10 - This register has no meaning in and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn sai_aslotr(&self) -> &SAI_ASLOTR {
        &self.sai_aslotr
    }
    #[doc = "0x14 - Interrupt mask register"]
    #[inline(always)]
    pub const fn sai_aim(&self) -> &SAI_AIM {
        &self.sai_aim
    }
    #[doc = "0x18 - Status register"]
    #[inline(always)]
    pub const fn sai_asr(&self) -> &SAI_ASR {
        &self.sai_asr
    }
    #[doc = "0x1c - Clear flag register"]
    #[inline(always)]
    pub const fn sai_aclrfr(&self) -> &SAI_ACLRFR {
        &self.sai_aclrfr
    }
    #[doc = "0x20 - Data register"]
    #[inline(always)]
    pub const fn sai_adr(&self) -> &SAI_ADR {
        &self.sai_adr
    }
    #[doc = "0x24 - Configuration register 1"]
    #[inline(always)]
    pub const fn sai_bcr1(&self) -> &SAI_BCR1 {
        &self.sai_bcr1
    }
    #[doc = "0x28 - Configuration register 2"]
    #[inline(always)]
    pub const fn sai_bcr2(&self) -> &SAI_BCR2 {
        &self.sai_bcr2
    }
    #[doc = "0x2c - This register has no meaning in and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn sai_bfrcr(&self) -> &SAI_BFRCR {
        &self.sai_bfrcr
    }
    #[doc = "0x30 - This register has no meaning in and SPDIF audio protocol"]
    #[inline(always)]
    pub const fn sai_bslotr(&self) -> &SAI_BSLOTR {
        &self.sai_bslotr
    }
    #[doc = "0x34 - Interrupt mask register"]
    #[inline(always)]
    pub const fn sai_bim(&self) -> &SAI_BIM {
        &self.sai_bim
    }
    #[doc = "0x38 - Status register"]
    #[inline(always)]
    pub const fn sai_bsr(&self) -> &SAI_BSR {
        &self.sai_bsr
    }
    #[doc = "0x3c - Clear flag register"]
    #[inline(always)]
    pub const fn sai_bclrfr(&self) -> &SAI_BCLRFR {
        &self.sai_bclrfr
    }
    #[doc = "0x40 - Data register"]
    #[inline(always)]
    pub const fn sai_bdr(&self) -> &SAI_BDR {
        &self.sai_bdr
    }
    #[doc = "0x44 - PDM control register"]
    #[inline(always)]
    pub const fn sai_pdmcr(&self) -> &SAI_PDMCR {
        &self.sai_pdmcr
    }
    #[doc = "0x48 - PDM delay register"]
    #[inline(always)]
    pub const fn sai_pdmdly(&self) -> &SAI_PDMDLY {
        &self.sai_pdmdly
    }
    #[doc = "0x3f0 - SAI hardware configuration register"]
    #[inline(always)]
    pub const fn sai_hwcfgr(&self) -> &SAI_HWCFGR {
        &self.sai_hwcfgr
    }
    #[doc = "0x3f4 - SAI version register"]
    #[inline(always)]
    pub const fn sai_verr(&self) -> &SAI_VERR {
        &self.sai_verr
    }
    #[doc = "0x3f8 - SAI identification register"]
    #[inline(always)]
    pub const fn sai_ipidr(&self) -> &SAI_IPIDR {
        &self.sai_ipidr
    }
    #[doc = "0x3fc - SAI size identification register"]
    #[inline(always)]
    pub const fn sai_sidr(&self) -> &SAI_SIDR {
        &self.sai_sidr
    }
}
#[doc = "SAI_GCR (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_gcr`]
module"]
pub type SAI_GCR = crate::Reg<sai_gcr::SAI_GCRrs>;
#[doc = "Global configuration register"]
pub mod sai_gcr;
#[doc = "SAI_ACR1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_acr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_acr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_acr1`]
module"]
pub type SAI_ACR1 = crate::Reg<sai_acr1::SAI_ACR1rs>;
#[doc = "Configuration register 1"]
pub mod sai_acr1;
#[doc = "SAI_ACR2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_acr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_acr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_acr2`]
module"]
pub type SAI_ACR2 = crate::Reg<sai_acr2::SAI_ACR2rs>;
#[doc = "Configuration register 2"]
pub mod sai_acr2;
#[doc = "SAI_AFRCR (rw) register accessor: This register has no meaning in and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_afrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_afrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_afrcr`]
module"]
pub type SAI_AFRCR = crate::Reg<sai_afrcr::SAI_AFRCRrs>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_afrcr;
#[doc = "SAI_ASLOTR (rw) register accessor: This register has no meaning in and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_aslotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_aslotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_aslotr`]
module"]
pub type SAI_ASLOTR = crate::Reg<sai_aslotr::SAI_ASLOTRrs>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_aslotr;
#[doc = "SAI_AIM (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_aim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_aim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_aim`]
module"]
pub type SAI_AIM = crate::Reg<sai_aim::SAI_AIMrs>;
#[doc = "Interrupt mask register"]
pub mod sai_aim;
#[doc = "SAI_ASR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_asr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_asr`]
module"]
pub type SAI_ASR = crate::Reg<sai_asr::SAI_ASRrs>;
#[doc = "Status register"]
pub mod sai_asr;
#[doc = "SAI_ACLRFR (w) register accessor: Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_aclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_aclrfr`]
module"]
pub type SAI_ACLRFR = crate::Reg<sai_aclrfr::SAI_ACLRFRrs>;
#[doc = "Clear flag register"]
pub mod sai_aclrfr;
#[doc = "SAI_ADR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_adr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_adr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_adr`]
module"]
pub type SAI_ADR = crate::Reg<sai_adr::SAI_ADRrs>;
#[doc = "Data register"]
pub mod sai_adr;
#[doc = "SAI_BCR1 (rw) register accessor: Configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bcr1`]
module"]
pub type SAI_BCR1 = crate::Reg<sai_bcr1::SAI_BCR1rs>;
#[doc = "Configuration register 1"]
pub mod sai_bcr1;
#[doc = "SAI_BCR2 (rw) register accessor: Configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bcr2`]
module"]
pub type SAI_BCR2 = crate::Reg<sai_bcr2::SAI_BCR2rs>;
#[doc = "Configuration register 2"]
pub mod sai_bcr2;
#[doc = "SAI_BFRCR (rw) register accessor: This register has no meaning in and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bfrcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bfrcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bfrcr`]
module"]
pub type SAI_BFRCR = crate::Reg<sai_bfrcr::SAI_BFRCRrs>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_bfrcr;
#[doc = "SAI_BSLOTR (rw) register accessor: This register has no meaning in and SPDIF audio protocol\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bslotr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bslotr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bslotr`]
module"]
pub type SAI_BSLOTR = crate::Reg<sai_bslotr::SAI_BSLOTRrs>;
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_bslotr;
#[doc = "SAI_BIM (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bim`]
module"]
pub type SAI_BIM = crate::Reg<sai_bim::SAI_BIMrs>;
#[doc = "Interrupt mask register"]
pub mod sai_bim;
#[doc = "SAI_BSR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bsr`]
module"]
pub type SAI_BSR = crate::Reg<sai_bsr::SAI_BSRrs>;
#[doc = "Status register"]
pub mod sai_bsr;
#[doc = "SAI_BCLRFR (w) register accessor: Clear flag register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bclrfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bclrfr`]
module"]
pub type SAI_BCLRFR = crate::Reg<sai_bclrfr::SAI_BCLRFRrs>;
#[doc = "Clear flag register"]
pub mod sai_bclrfr;
#[doc = "SAI_BDR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_bdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_bdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_bdr`]
module"]
pub type SAI_BDR = crate::Reg<sai_bdr::SAI_BDRrs>;
#[doc = "Data register"]
pub mod sai_bdr;
#[doc = "SAI_PDMCR (rw) register accessor: PDM control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_pdmcr`]
module"]
pub type SAI_PDMCR = crate::Reg<sai_pdmcr::SAI_PDMCRrs>;
#[doc = "PDM control register"]
pub mod sai_pdmcr;
#[doc = "SAI_PDMDLY (rw) register accessor: PDM delay register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_pdmdly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_pdmdly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_pdmdly`]
module"]
pub type SAI_PDMDLY = crate::Reg<sai_pdmdly::SAI_PDMDLYrs>;
#[doc = "PDM delay register"]
pub mod sai_pdmdly;
#[doc = "SAI_HWCFGR (r) register accessor: SAI hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_hwcfgr`]
module"]
pub type SAI_HWCFGR = crate::Reg<sai_hwcfgr::SAI_HWCFGRrs>;
#[doc = "SAI hardware configuration register"]
pub mod sai_hwcfgr;
#[doc = "SAI_VERR (r) register accessor: SAI version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_verr`]
module"]
pub type SAI_VERR = crate::Reg<sai_verr::SAI_VERRrs>;
#[doc = "SAI version register"]
pub mod sai_verr;
#[doc = "SAI_IPIDR (r) register accessor: SAI identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_ipidr`]
module"]
pub type SAI_IPIDR = crate::Reg<sai_ipidr::SAI_IPIDRrs>;
#[doc = "SAI identification register"]
pub mod sai_ipidr;
#[doc = "SAI_SIDR (r) register accessor: SAI size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sai_sidr`]
module"]
pub type SAI_SIDR = crate::Reg<sai_sidr::SAI_SIDRrs>;
#[doc = "SAI size identification register"]
pub mod sai_sidr;
