#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    quadspi_cr: QUADSPI_CR,
    quadspi_dcr: QUADSPI_DCR,
    quadspi_sr: QUADSPI_SR,
    quadspi_fcr: QUADSPI_FCR,
    quadspi_dlr: QUADSPI_DLR,
    quadspi_ccr: QUADSPI_CCR,
    quadspi_ar: QUADSPI_AR,
    quadspi_abr: QUADSPI_ABR,
    quadspi_dr: QUADSPI_DR,
    quadspi_psmkr: QUADSPI_PSMKR,
    quadspi_psmar: QUADSPI_PSMAR,
    quadspi_pir: QUADSPI_PIR,
    quadspi_lptr: QUADSPI_LPTR,
    _reserved13: [u8; 0x03bc],
    quadspi_hwcfgr: QUADSPI_HWCFGR,
    quadspi_verr: QUADSPI_VERR,
    quadspi_ipidr: QUADSPI_IPIDR,
    quadspi_sidr: QUADSPI_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    #[inline(always)]
    pub const fn quadspi_cr(&self) -> &QUADSPI_CR {
        &self.quadspi_cr
    }
    #[doc = "0x04 - QUADSPI device configuration register"]
    #[inline(always)]
    pub const fn quadspi_dcr(&self) -> &QUADSPI_DCR {
        &self.quadspi_dcr
    }
    #[doc = "0x08 - QUADSPI status register"]
    #[inline(always)]
    pub const fn quadspi_sr(&self) -> &QUADSPI_SR {
        &self.quadspi_sr
    }
    #[doc = "0x0c - QUADSPI flag clear register"]
    #[inline(always)]
    pub const fn quadspi_fcr(&self) -> &QUADSPI_FCR {
        &self.quadspi_fcr
    }
    #[doc = "0x10 - QUADSPI data length register"]
    #[inline(always)]
    pub const fn quadspi_dlr(&self) -> &QUADSPI_DLR {
        &self.quadspi_dlr
    }
    #[doc = "0x14 - QUADSPI communication configuration register"]
    #[inline(always)]
    pub const fn quadspi_ccr(&self) -> &QUADSPI_CCR {
        &self.quadspi_ccr
    }
    #[doc = "0x18 - QUADSPI address register"]
    #[inline(always)]
    pub const fn quadspi_ar(&self) -> &QUADSPI_AR {
        &self.quadspi_ar
    }
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    #[inline(always)]
    pub const fn quadspi_abr(&self) -> &QUADSPI_ABR {
        &self.quadspi_abr
    }
    #[doc = "0x20 - QUADSPI data register"]
    #[inline(always)]
    pub const fn quadspi_dr(&self) -> &QUADSPI_DR {
        &self.quadspi_dr
    }
    #[doc = "0x24 - QUADSPI polling status mask register"]
    #[inline(always)]
    pub const fn quadspi_psmkr(&self) -> &QUADSPI_PSMKR {
        &self.quadspi_psmkr
    }
    #[doc = "0x28 - QUADSPI polling status match register"]
    #[inline(always)]
    pub const fn quadspi_psmar(&self) -> &QUADSPI_PSMAR {
        &self.quadspi_psmar
    }
    #[doc = "0x2c - QUADSPI polling interval register"]
    #[inline(always)]
    pub const fn quadspi_pir(&self) -> &QUADSPI_PIR {
        &self.quadspi_pir
    }
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    #[inline(always)]
    pub const fn quadspi_lptr(&self) -> &QUADSPI_LPTR {
        &self.quadspi_lptr
    }
    #[doc = "0x3f0 - QUADSPI HW configuration register"]
    #[inline(always)]
    pub const fn quadspi_hwcfgr(&self) -> &QUADSPI_HWCFGR {
        &self.quadspi_hwcfgr
    }
    #[doc = "0x3f4 - QUADSPI version register"]
    #[inline(always)]
    pub const fn quadspi_verr(&self) -> &QUADSPI_VERR {
        &self.quadspi_verr
    }
    #[doc = "0x3f8 - QUADSPI identification register"]
    #[inline(always)]
    pub const fn quadspi_ipidr(&self) -> &QUADSPI_IPIDR {
        &self.quadspi_ipidr
    }
    #[doc = "0x3fc - QUADSPI size identification register"]
    #[inline(always)]
    pub const fn quadspi_sidr(&self) -> &QUADSPI_SIDR {
        &self.quadspi_sidr
    }
}
#[doc = "QUADSPI_CR (rw) register accessor: QUADSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_cr`]
module"]
pub type QUADSPI_CR = crate::Reg<quadspi_cr::QUADSPI_CRrs>;
#[doc = "QUADSPI control register"]
pub mod quadspi_cr;
#[doc = "QUADSPI_DCR (rw) register accessor: QUADSPI device configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_dcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_dcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_dcr`]
module"]
pub type QUADSPI_DCR = crate::Reg<quadspi_dcr::QUADSPI_DCRrs>;
#[doc = "QUADSPI device configuration register"]
pub mod quadspi_dcr;
#[doc = "QUADSPI_SR (r) register accessor: QUADSPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_sr`]
module"]
pub type QUADSPI_SR = crate::Reg<quadspi_sr::QUADSPI_SRrs>;
#[doc = "QUADSPI status register"]
pub mod quadspi_sr;
#[doc = "QUADSPI_FCR (w) register accessor: QUADSPI flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_fcr`]
module"]
pub type QUADSPI_FCR = crate::Reg<quadspi_fcr::QUADSPI_FCRrs>;
#[doc = "QUADSPI flag clear register"]
pub mod quadspi_fcr;
#[doc = "QUADSPI_DLR (rw) register accessor: QUADSPI data length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_dlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_dlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_dlr`]
module"]
pub type QUADSPI_DLR = crate::Reg<quadspi_dlr::QUADSPI_DLRrs>;
#[doc = "QUADSPI data length register"]
pub mod quadspi_dlr;
#[doc = "QUADSPI_CCR (rw) register accessor: QUADSPI communication configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_ccr`]
module"]
pub type QUADSPI_CCR = crate::Reg<quadspi_ccr::QUADSPI_CCRrs>;
#[doc = "QUADSPI communication configuration register"]
pub mod quadspi_ccr;
#[doc = "QUADSPI_AR (rw) register accessor: QUADSPI address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_ar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_ar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_ar`]
module"]
pub type QUADSPI_AR = crate::Reg<quadspi_ar::QUADSPI_ARrs>;
#[doc = "QUADSPI address register"]
pub mod quadspi_ar;
#[doc = "QUADSPI_ABR (rw) register accessor: QUADSPI alternate bytes registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_abr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_abr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_abr`]
module"]
pub type QUADSPI_ABR = crate::Reg<quadspi_abr::QUADSPI_ABRrs>;
#[doc = "QUADSPI alternate bytes registers"]
pub mod quadspi_abr;
#[doc = "QUADSPI_DR (rw) register accessor: QUADSPI data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_dr`]
module"]
pub type QUADSPI_DR = crate::Reg<quadspi_dr::QUADSPI_DRrs>;
#[doc = "QUADSPI data register"]
pub mod quadspi_dr;
#[doc = "QUADSPI_PSMKR (rw) register accessor: QUADSPI polling status mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_psmkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_psmkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_psmkr`]
module"]
pub type QUADSPI_PSMKR = crate::Reg<quadspi_psmkr::QUADSPI_PSMKRrs>;
#[doc = "QUADSPI polling status mask register"]
pub mod quadspi_psmkr;
#[doc = "QUADSPI_PSMAR (rw) register accessor: QUADSPI polling status match register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_psmar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_psmar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_psmar`]
module"]
pub type QUADSPI_PSMAR = crate::Reg<quadspi_psmar::QUADSPI_PSMARrs>;
#[doc = "QUADSPI polling status match register"]
pub mod quadspi_psmar;
#[doc = "QUADSPI_PIR (rw) register accessor: QUADSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_pir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_pir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_pir`]
module"]
pub type QUADSPI_PIR = crate::Reg<quadspi_pir::QUADSPI_PIRrs>;
#[doc = "QUADSPI polling interval register"]
pub mod quadspi_pir;
#[doc = "QUADSPI_LPTR (rw) register accessor: QUADSPI low-power timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_lptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_lptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_lptr`]
module"]
pub type QUADSPI_LPTR = crate::Reg<quadspi_lptr::QUADSPI_LPTRrs>;
#[doc = "QUADSPI low-power timeout register"]
pub mod quadspi_lptr;
#[doc = "QUADSPI_HWCFGR (r) register accessor: QUADSPI HW configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_hwcfgr`]
module"]
pub type QUADSPI_HWCFGR = crate::Reg<quadspi_hwcfgr::QUADSPI_HWCFGRrs>;
#[doc = "QUADSPI HW configuration register"]
pub mod quadspi_hwcfgr;
#[doc = "QUADSPI_VERR (r) register accessor: QUADSPI version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_verr`]
module"]
pub type QUADSPI_VERR = crate::Reg<quadspi_verr::QUADSPI_VERRrs>;
#[doc = "QUADSPI version register"]
pub mod quadspi_verr;
#[doc = "QUADSPI_IPIDR (r) register accessor: QUADSPI identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_ipidr`]
module"]
pub type QUADSPI_IPIDR = crate::Reg<quadspi_ipidr::QUADSPI_IPIDRrs>;
#[doc = "QUADSPI identification register"]
pub mod quadspi_ipidr;
#[doc = "QUADSPI_SIDR (r) register accessor: QUADSPI size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@quadspi_sidr`]
module"]
pub type QUADSPI_SIDR = crate::Reg<quadspi_sidr::QUADSPI_SIDRrs>;
#[doc = "QUADSPI size identification register"]
pub mod quadspi_sidr;
