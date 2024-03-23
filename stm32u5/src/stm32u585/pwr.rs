#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwr_cr1: PWR_CR1,
    pwr_cr2: PWR_CR2,
    pwr_cr3: PWR_CR3,
    pwr_vosr: PWR_VOSR,
    pwr_svmcr: PWR_SVMCR,
    pwr_wucr1: PWR_WUCR1,
    pwr_wucr2: PWR_WUCR2,
    pwr_wucr3: PWR_WUCR3,
    pwr_bdcr1: PWR_BDCR1,
    pwr_bdcr2: PWR_BDCR2,
    pwr_dbpr: PWR_DBPR,
    pwr_ucpdr: PWR_UCPDR,
    pwr_seccfgr: PWR_SECCFGR,
    pwr_privcfgr: PWR_PRIVCFGR,
    pwr_sr: PWR_SR,
    pwr_svmsr: PWR_SVMSR,
    pwr_bdsr: PWR_BDSR,
    pwr_wusr: PWR_WUSR,
    pwr_wuscr: PWR_WUSCR,
    pwr_apcr: PWR_APCR,
    pwr_pucra: PWR_PUCRA,
    pwr_pdcra: PWR_PDCRA,
    pwr_pucrb: PWR_PUCRB,
    pwr_pdcrb: PWR_PDCRB,
    pwr_pucrc: PWR_PUCRC,
    pwr_pdcrc: PWR_PDCRC,
    pwr_pucrd: PWR_PUCRD,
    pwr_pdcrd: PWR_PDCRD,
    pwr_pucre: PWR_PUCRE,
    pwr_pdcre: PWR_PDCRE,
    pwr_pucrf: PWR_PUCRF,
    pwr_pdcrf: PWR_PDCRF,
    pwr_pucrg: PWR_PUCRG,
    pwr_pdcrg: PWR_PDCRG,
    pwr_pucrh: PWR_PUCRH,
    pwr_pdcrh: PWR_PDCRH,
    pwr_pucri: PWR_PUCRI,
    pwr_pdcri: PWR_PDCRI,
}
impl RegisterBlock {
    #[doc = "0x00 - PWR control register 1"]
    #[inline(always)]
    pub const fn pwr_cr1(&self) -> &PWR_CR1 {
        &self.pwr_cr1
    }
    #[doc = "0x04 - PWR control register 2"]
    #[inline(always)]
    pub const fn pwr_cr2(&self) -> &PWR_CR2 {
        &self.pwr_cr2
    }
    #[doc = "0x08 - PWR control register 3"]
    #[inline(always)]
    pub const fn pwr_cr3(&self) -> &PWR_CR3 {
        &self.pwr_cr3
    }
    #[doc = "0x0c - PWR voltage scaling register"]
    #[inline(always)]
    pub const fn pwr_vosr(&self) -> &PWR_VOSR {
        &self.pwr_vosr
    }
    #[doc = "0x10 - PWR supply voltage monitoring control register"]
    #[inline(always)]
    pub const fn pwr_svmcr(&self) -> &PWR_SVMCR {
        &self.pwr_svmcr
    }
    #[doc = "0x14 - PWR wakeup control register 1"]
    #[inline(always)]
    pub const fn pwr_wucr1(&self) -> &PWR_WUCR1 {
        &self.pwr_wucr1
    }
    #[doc = "0x18 - PWR wakeup control register 2"]
    #[inline(always)]
    pub const fn pwr_wucr2(&self) -> &PWR_WUCR2 {
        &self.pwr_wucr2
    }
    #[doc = "0x1c - PWR wakeup control register 3"]
    #[inline(always)]
    pub const fn pwr_wucr3(&self) -> &PWR_WUCR3 {
        &self.pwr_wucr3
    }
    #[doc = "0x20 - PWR Backup domain control register 1"]
    #[inline(always)]
    pub const fn pwr_bdcr1(&self) -> &PWR_BDCR1 {
        &self.pwr_bdcr1
    }
    #[doc = "0x24 - PWR Backup domain control register 2"]
    #[inline(always)]
    pub const fn pwr_bdcr2(&self) -> &PWR_BDCR2 {
        &self.pwr_bdcr2
    }
    #[doc = "0x28 - PWR disable Backup domain register"]
    #[inline(always)]
    pub const fn pwr_dbpr(&self) -> &PWR_DBPR {
        &self.pwr_dbpr
    }
    #[doc = "0x2c - PWR USB Type-C™ and Power Delivery register"]
    #[inline(always)]
    pub const fn pwr_ucpdr(&self) -> &PWR_UCPDR {
        &self.pwr_ucpdr
    }
    #[doc = "0x30 - PWR security configuration register"]
    #[inline(always)]
    pub const fn pwr_seccfgr(&self) -> &PWR_SECCFGR {
        &self.pwr_seccfgr
    }
    #[doc = "0x34 - PWR privilege control register"]
    #[inline(always)]
    pub const fn pwr_privcfgr(&self) -> &PWR_PRIVCFGR {
        &self.pwr_privcfgr
    }
    #[doc = "0x38 - PWR status register"]
    #[inline(always)]
    pub const fn pwr_sr(&self) -> &PWR_SR {
        &self.pwr_sr
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn pwr_svmsr(&self) -> &PWR_SVMSR {
        &self.pwr_svmsr
    }
    #[doc = "0x40 - PWR Backup domain status register"]
    #[inline(always)]
    pub const fn pwr_bdsr(&self) -> &PWR_BDSR {
        &self.pwr_bdsr
    }
    #[doc = "0x44 - PWR wakeup status register"]
    #[inline(always)]
    pub const fn pwr_wusr(&self) -> &PWR_WUSR {
        &self.pwr_wusr
    }
    #[doc = "0x48 - PWR wakeup status clear register"]
    #[inline(always)]
    pub const fn pwr_wuscr(&self) -> &PWR_WUSCR {
        &self.pwr_wuscr
    }
    #[doc = "0x4c - PWR apply pull configuration register"]
    #[inline(always)]
    pub const fn pwr_apcr(&self) -> &PWR_APCR {
        &self.pwr_apcr
    }
    #[doc = "0x50 - PWR port A pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucra(&self) -> &PWR_PUCRA {
        &self.pwr_pucra
    }
    #[doc = "0x54 - PWR port A pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcra(&self) -> &PWR_PDCRA {
        &self.pwr_pdcra
    }
    #[doc = "0x58 - PWR port B pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrb(&self) -> &PWR_PUCRB {
        &self.pwr_pucrb
    }
    #[doc = "0x5c - PWR port B pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrb(&self) -> &PWR_PDCRB {
        &self.pwr_pdcrb
    }
    #[doc = "0x60 - PWR port C pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrc(&self) -> &PWR_PUCRC {
        &self.pwr_pucrc
    }
    #[doc = "0x64 - PWR port C pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrc(&self) -> &PWR_PDCRC {
        &self.pwr_pdcrc
    }
    #[doc = "0x68 - PWR port D pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrd(&self) -> &PWR_PUCRD {
        &self.pwr_pucrd
    }
    #[doc = "0x6c - PWR port D pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrd(&self) -> &PWR_PDCRD {
        &self.pwr_pdcrd
    }
    #[doc = "0x70 - PWR port E pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucre(&self) -> &PWR_PUCRE {
        &self.pwr_pucre
    }
    #[doc = "0x74 - PWR port E pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcre(&self) -> &PWR_PDCRE {
        &self.pwr_pdcre
    }
    #[doc = "0x78 - PWR port F pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrf(&self) -> &PWR_PUCRF {
        &self.pwr_pucrf
    }
    #[doc = "0x7c - PWR port F pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrf(&self) -> &PWR_PDCRF {
        &self.pwr_pdcrf
    }
    #[doc = "0x80 - PWR port G pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrg(&self) -> &PWR_PUCRG {
        &self.pwr_pucrg
    }
    #[doc = "0x84 - PWR port G pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrg(&self) -> &PWR_PDCRG {
        &self.pwr_pdcrg
    }
    #[doc = "0x88 - PWR port H pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucrh(&self) -> &PWR_PUCRH {
        &self.pwr_pucrh
    }
    #[doc = "0x8c - PWR port H pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcrh(&self) -> &PWR_PDCRH {
        &self.pwr_pdcrh
    }
    #[doc = "0x90 - PWR port I pull-up control register"]
    #[inline(always)]
    pub const fn pwr_pucri(&self) -> &PWR_PUCRI {
        &self.pwr_pucri
    }
    #[doc = "0x94 - PWR port I pull-down control register"]
    #[inline(always)]
    pub const fn pwr_pdcri(&self) -> &PWR_PDCRI {
        &self.pwr_pdcri
    }
}
#[doc = "PWR_CR1 (rw) register accessor: PWR control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr1`]
module"]
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1rs>;
#[doc = "PWR control register 1"]
pub mod pwr_cr1;
#[doc = "PWR_CR2 (rw) register accessor: PWR control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr2`]
module"]
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2rs>;
#[doc = "PWR control register 2"]
pub mod pwr_cr2;
#[doc = "PWR_CR3 (rw) register accessor: PWR control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_cr3`]
module"]
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3rs>;
#[doc = "PWR control register 3"]
pub mod pwr_cr3;
#[doc = "PWR_VOSR (rw) register accessor: PWR voltage scaling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_vosr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_vosr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_vosr`]
module"]
pub type PWR_VOSR = crate::Reg<pwr_vosr::PWR_VOSRrs>;
#[doc = "PWR voltage scaling register"]
pub mod pwr_vosr;
#[doc = "PWR_SVMCR (rw) register accessor: PWR supply voltage monitoring control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_svmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_svmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_svmcr`]
module"]
pub type PWR_SVMCR = crate::Reg<pwr_svmcr::PWR_SVMCRrs>;
#[doc = "PWR supply voltage monitoring control register"]
pub mod pwr_svmcr;
#[doc = "PWR_WUCR1 (rw) register accessor: PWR wakeup control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wucr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_wucr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wucr1`]
module"]
pub type PWR_WUCR1 = crate::Reg<pwr_wucr1::PWR_WUCR1rs>;
#[doc = "PWR wakeup control register 1"]
pub mod pwr_wucr1;
#[doc = "PWR_WUCR2 (rw) register accessor: PWR wakeup control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wucr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_wucr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wucr2`]
module"]
pub type PWR_WUCR2 = crate::Reg<pwr_wucr2::PWR_WUCR2rs>;
#[doc = "PWR wakeup control register 2"]
pub mod pwr_wucr2;
#[doc = "PWR_WUCR3 (rw) register accessor: PWR wakeup control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wucr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_wucr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wucr3`]
module"]
pub type PWR_WUCR3 = crate::Reg<pwr_wucr3::PWR_WUCR3rs>;
#[doc = "PWR wakeup control register 3"]
pub mod pwr_wucr3;
#[doc = "PWR_BDCR1 (rw) register accessor: PWR Backup domain control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_bdcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_bdcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_bdcr1`]
module"]
pub type PWR_BDCR1 = crate::Reg<pwr_bdcr1::PWR_BDCR1rs>;
#[doc = "PWR Backup domain control register 1"]
pub mod pwr_bdcr1;
#[doc = "PWR_BDCR2 (rw) register accessor: PWR Backup domain control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_bdcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_bdcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_bdcr2`]
module"]
pub type PWR_BDCR2 = crate::Reg<pwr_bdcr2::PWR_BDCR2rs>;
#[doc = "PWR Backup domain control register 2"]
pub mod pwr_bdcr2;
#[doc = "PWR_DBPR (rw) register accessor: PWR disable Backup domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_dbpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_dbpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_dbpr`]
module"]
pub type PWR_DBPR = crate::Reg<pwr_dbpr::PWR_DBPRrs>;
#[doc = "PWR disable Backup domain register"]
pub mod pwr_dbpr;
#[doc = "PWR_UCPDR (rw) register accessor: PWR USB Type-C™ and Power Delivery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_ucpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_ucpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_ucpdr`]
module"]
pub type PWR_UCPDR = crate::Reg<pwr_ucpdr::PWR_UCPDRrs>;
#[doc = "PWR USB Type-C™ and Power Delivery register"]
pub mod pwr_ucpdr;
#[doc = "PWR_SECCFGR (rw) register accessor: PWR security configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_seccfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_seccfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_seccfgr`]
module"]
pub type PWR_SECCFGR = crate::Reg<pwr_seccfgr::PWR_SECCFGRrs>;
#[doc = "PWR security configuration register"]
pub mod pwr_seccfgr;
#[doc = "PWR_PRIVCFGR (rw) register accessor: PWR privilege control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_privcfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_privcfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_privcfgr`]
module"]
pub type PWR_PRIVCFGR = crate::Reg<pwr_privcfgr::PWR_PRIVCFGRrs>;
#[doc = "PWR privilege control register"]
pub mod pwr_privcfgr;
#[doc = "PWR_SR (rw) register accessor: PWR status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_sr`]
module"]
pub type PWR_SR = crate::Reg<pwr_sr::PWR_SRrs>;
#[doc = "PWR status register"]
pub mod pwr_sr;
#[doc = "PWR_SVMSR (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_svmsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_svmsr`]
module"]
pub type PWR_SVMSR = crate::Reg<pwr_svmsr::PWR_SVMSRrs>;
#[doc = ""]
pub mod pwr_svmsr;
#[doc = "PWR_BDSR (r) register accessor: PWR Backup domain status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_bdsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_bdsr`]
module"]
pub type PWR_BDSR = crate::Reg<pwr_bdsr::PWR_BDSRrs>;
#[doc = "PWR Backup domain status register"]
pub mod pwr_bdsr;
#[doc = "PWR_WUSR (r) register accessor: PWR wakeup status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wusr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wusr`]
module"]
pub type PWR_WUSR = crate::Reg<pwr_wusr::PWR_WUSRrs>;
#[doc = "PWR wakeup status register"]
pub mod pwr_wusr;
#[doc = "PWR_WUSCR (w) register accessor: PWR wakeup status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_wuscr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_wuscr`]
module"]
pub type PWR_WUSCR = crate::Reg<pwr_wuscr::PWR_WUSCRrs>;
#[doc = "PWR wakeup status clear register"]
pub mod pwr_wuscr;
#[doc = "PWR_APCR (rw) register accessor: PWR apply pull configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_apcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_apcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_apcr`]
module"]
pub type PWR_APCR = crate::Reg<pwr_apcr::PWR_APCRrs>;
#[doc = "PWR apply pull configuration register"]
pub mod pwr_apcr;
#[doc = "PWR_PUCRA (rw) register accessor: PWR port A pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucra`]
module"]
pub type PWR_PUCRA = crate::Reg<pwr_pucra::PWR_PUCRArs>;
#[doc = "PWR port A pull-up control register"]
pub mod pwr_pucra;
#[doc = "PWR_PDCRA (rw) register accessor: PWR port A pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcra`]
module"]
pub type PWR_PDCRA = crate::Reg<pwr_pdcra::PWR_PDCRArs>;
#[doc = "PWR port A pull-down control register"]
pub mod pwr_pdcra;
#[doc = "PWR_PUCRB (rw) register accessor: PWR port B pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrb`]
module"]
pub type PWR_PUCRB = crate::Reg<pwr_pucrb::PWR_PUCRBrs>;
#[doc = "PWR port B pull-up control register"]
pub mod pwr_pucrb;
#[doc = "PWR_PDCRB (rw) register accessor: PWR port B pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrb`]
module"]
pub type PWR_PDCRB = crate::Reg<pwr_pdcrb::PWR_PDCRBrs>;
#[doc = "PWR port B pull-down control register"]
pub mod pwr_pdcrb;
#[doc = "PWR_PUCRC (rw) register accessor: PWR port C pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrc`]
module"]
pub type PWR_PUCRC = crate::Reg<pwr_pucrc::PWR_PUCRCrs>;
#[doc = "PWR port C pull-up control register"]
pub mod pwr_pucrc;
#[doc = "PWR_PDCRC (rw) register accessor: PWR port C pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrc`]
module"]
pub type PWR_PDCRC = crate::Reg<pwr_pdcrc::PWR_PDCRCrs>;
#[doc = "PWR port C pull-down control register"]
pub mod pwr_pdcrc;
#[doc = "PWR_PUCRD (rw) register accessor: PWR port D pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrd`]
module"]
pub type PWR_PUCRD = crate::Reg<pwr_pucrd::PWR_PUCRDrs>;
#[doc = "PWR port D pull-up control register"]
pub mod pwr_pucrd;
#[doc = "PWR_PDCRD (rw) register accessor: PWR port D pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrd`]
module"]
pub type PWR_PDCRD = crate::Reg<pwr_pdcrd::PWR_PDCRDrs>;
#[doc = "PWR port D pull-down control register"]
pub mod pwr_pdcrd;
#[doc = "PWR_PUCRE (rw) register accessor: PWR port E pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucre`]
module"]
pub type PWR_PUCRE = crate::Reg<pwr_pucre::PWR_PUCRErs>;
#[doc = "PWR port E pull-up control register"]
pub mod pwr_pucre;
#[doc = "PWR_PDCRE (rw) register accessor: PWR port E pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcre`]
module"]
pub type PWR_PDCRE = crate::Reg<pwr_pdcre::PWR_PDCRErs>;
#[doc = "PWR port E pull-down control register"]
pub mod pwr_pdcre;
#[doc = "PWR_PUCRF (rw) register accessor: PWR port F pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrf`]
module"]
pub type PWR_PUCRF = crate::Reg<pwr_pucrf::PWR_PUCRFrs>;
#[doc = "PWR port F pull-up control register"]
pub mod pwr_pucrf;
#[doc = "PWR_PDCRF (rw) register accessor: PWR port F pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrf`]
module"]
pub type PWR_PDCRF = crate::Reg<pwr_pdcrf::PWR_PDCRFrs>;
#[doc = "PWR port F pull-down control register"]
pub mod pwr_pdcrf;
#[doc = "PWR_PUCRG (rw) register accessor: PWR port G pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrg`]
module"]
pub type PWR_PUCRG = crate::Reg<pwr_pucrg::PWR_PUCRGrs>;
#[doc = "PWR port G pull-up control register"]
pub mod pwr_pucrg;
#[doc = "PWR_PDCRG (rw) register accessor: PWR port G pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrg`]
module"]
pub type PWR_PDCRG = crate::Reg<pwr_pdcrg::PWR_PDCRGrs>;
#[doc = "PWR port G pull-down control register"]
pub mod pwr_pdcrg;
#[doc = "PWR_PUCRH (rw) register accessor: PWR port H pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucrh`]
module"]
pub type PWR_PUCRH = crate::Reg<pwr_pucrh::PWR_PUCRHrs>;
#[doc = "PWR port H pull-up control register"]
pub mod pwr_pucrh;
#[doc = "PWR_PDCRH (rw) register accessor: PWR port H pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcrh`]
module"]
pub type PWR_PDCRH = crate::Reg<pwr_pdcrh::PWR_PDCRHrs>;
#[doc = "PWR port H pull-down control register"]
pub mod pwr_pdcrh;
#[doc = "PWR_PUCRI (rw) register accessor: PWR port I pull-up control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pucri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pucri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pucri`]
module"]
pub type PWR_PUCRI = crate::Reg<pwr_pucri::PWR_PUCRIrs>;
#[doc = "PWR port I pull-up control register"]
pub mod pwr_pucri;
#[doc = "PWR_PDCRI (rw) register accessor: PWR port I pull-down control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_pdcri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_pdcri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_pdcri`]
module"]
pub type PWR_PDCRI = crate::Reg<pwr_pdcri::PWR_PDCRIrs>;
#[doc = "PWR port I pull-down control register"]
pub mod pwr_pdcri;
